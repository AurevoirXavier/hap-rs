use std::{sync::{Arc, Mutex}, ops::Deref};

use serde::ser::{Serialize, Serializer, SerializeStruct};
use erased_serde;

use accessory::HapAccessory;
use characteristic::Perm;
use transport::http::{Status, server::EventSubscriptions, handlers::characteristics::{
    ReadResponseObject, WriteObject, WriteResponseObject
}};
use event::EmitterPtr;

#[derive(Clone)]
pub struct AccessoryList {
    pub accessories: Arc<Mutex<Vec<Box<AccessoryListTrait>>>>,
}

impl AccessoryList {
    pub fn new(accessories: Vec<Box<AccessoryListTrait>>) -> AccessoryList {
        AccessoryList { accessories: Arc::new(Mutex::new(accessories)) }
    }

    pub fn init_aids(&mut self, event_emitter: EmitterPtr) {
        let mut next_aid = 1;
        let mut a = self.accessories.lock().unwrap();
        for accessory in a.iter_mut() {
            accessory.set_id(next_aid);
            accessory.init_iids(next_aid, event_emitter.clone());
            next_aid += 1;
        }
    }

    pub fn read_characteristic(
        &self,
        aid: u64,
        iid: u64,
        meta: bool,
        perms: bool,
        hap_type: bool,
        ev: bool,
    ) -> ReadResponseObject {
        let mut result_object = ReadResponseObject {
            iid,
            aid,
            hap_type: None,
            format: None,
            perms: None,
            ev: None,
            value: None,
            unit: None,
            max_value: None,
            min_value: None,
            step_value: None,
            max_len: None,
            status: Some(0),
        };

        let mut a = self.accessories.lock().unwrap();
        'l: for accessory in a.iter_mut() {
            if accessory.get_id() == aid {
                for service in accessory.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id() == iid {
                            match characteristic.get_value() {
                                Ok(value) => {
                                    result_object.value = Some(value);
                                    if meta {
                                        result_object.format = Some(characteristic.get_format().clone());
                                        result_object.unit = characteristic.get_unit().clone();
                                        result_object.max_value = characteristic.get_max_value();
                                        result_object.min_value = characteristic.get_min_value();
                                        result_object.step_value = characteristic.get_step_value();
                                        result_object.max_len = characteristic.get_max_len();
                                    }
                                    if perms {
                                        result_object.perms = Some(characteristic.get_perms().clone());
                                    }
                                    if hap_type {
                                        result_object.hap_type = Some(characteristic.get_type().clone());
                                    }
                                    if ev {
                                        result_object.ev = characteristic.get_event_notifications();
                                    }
                                },
                                Err(_) => {
                                    result_object.status = Some(Status::ServiceCommunicationFailure as i32);
                                },
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        result_object
    }

    pub fn write_characteristic(
        &self,
        write_object: WriteObject,
        event_subscriptions: &EventSubscriptions,
        event_emitter: &EmitterPtr,
    ) -> WriteResponseObject {
        let mut result_object = WriteResponseObject {
            aid: write_object.aid,
            iid: write_object.iid,
            status: 0,
        };

        let mut a = self.accessories.lock().unwrap();
        'l: for accessory in a.iter_mut() {
            if accessory.get_id() == write_object.aid {
                for service in accessory.get_mut_services() {
                    for characteristic in service.get_mut_characteristics() {
                        if characteristic.get_id() == write_object.iid {
                            // TODO - permission checking
                            // let perms = characteristic.get_perms();
                            // if perms.contains(&Perm::PairedWrite) {}
                            if let Some(value) = write_object.value {
                                // TODO - handle error
                                characteristic.set_value(value).unwrap();
                            }
                            if let Some(ev) = write_object.ev {
                                if characteristic.get_perms().contains(&Perm::Events) {
                                    characteristic.set_event_notifications(Some(ev));
                                    let subscription = (write_object.aid, write_object.iid);
                                    let mut es = event_subscriptions.lock().unwrap();
                                    let pos = es.iter().position(|&s| s == subscription);
                                    match (ev, pos) {
                                        (true, None) => { es.push(subscription); },
                                        (false, Some(p)) => { es.remove(p); },
                                        _ => {},
                                    }
                                } else {
                                    result_object.status = Status::NotificationNotSupported as i32;
                                }
                            }
                            break 'l;
                        }
                    }
                }
            }
        }

        result_object
    }
}

impl Serialize for AccessoryList {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut state = serializer.serialize_struct("AccessoryList", 1)?;
        let a = self.accessories.deref();
        state.serialize_field("accessories", &a)?;
        state.end()
    }
}

pub trait AccessoryListTrait: HapAccessory + erased_serde::Serialize {}

impl<T: HapAccessory + erased_serde::Serialize> AccessoryListTrait for T {}

serialize_trait_object!(AccessoryListTrait);