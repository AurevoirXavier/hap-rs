// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	accessory::{AccessoryInformation, HapAccessory},
	service::{
		accessory_information::AccessoryInformationService, smart_speaker::SmartSpeakerService,
		HapService,
	},
	HapType, Result,
};

/// Smart Speaker accessory.
#[derive(Debug, Default)]
pub struct SmartSpeakerAccessory {
	/// ID of the Smart Speaker accessory.
	id: u64,

	/// Accessory Information service.
	pub accessory_information: AccessoryInformationService,
	/// Smart Speaker service.
	pub smart_speaker: SmartSpeakerService,
}

impl SmartSpeakerAccessory {
	/// Creates a new Smart Speaker accessory.
	pub fn new(id: u64, information: AccessoryInformation) -> Result<Self> {
		let accessory_information = information.to_service(1, id)?;
		let smart_speaker_id = accessory_information.get_characteristics().len() as u64;
		let mut smart_speaker = SmartSpeakerService::new(1 + smart_speaker_id + 1, id);
		smart_speaker.set_primary(true);

		Ok(Self { id, accessory_information, smart_speaker })
	}
}

impl HapAccessory for SmartSpeakerAccessory {
	fn get_id(&self) -> u64 {
		self.id
	}

	fn set_id(&mut self, id: u64) {
		self.id = id;
	}

	fn get_service(&self, hap_type: HapType) -> Option<&dyn HapService> {
		self.get_services().into_iter().find(|&service| service.get_type() == hap_type)
	}

	fn get_mut_service(&mut self, hap_type: HapType) -> Option<&mut dyn HapService> {
		self.get_mut_services().into_iter().find(|service| service.get_type() == hap_type)
	}

	fn get_services(&self) -> Vec<&dyn HapService> {
		vec![&self.accessory_information, &self.smart_speaker]
	}

	fn get_mut_services(&mut self) -> Vec<&mut dyn HapService> {
		vec![&mut self.accessory_information, &mut self.smart_speaker]
	}
}

impl Serialize for SmartSpeakerAccessory {
	fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		let mut state = serializer.serialize_struct("HapAccessory", 2)?;
		state.serialize_field("aid", &self.get_id())?;
		state.serialize_field("services", &self.get_services())?;
		state.end()
	}
}
