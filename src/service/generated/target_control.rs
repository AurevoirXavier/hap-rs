// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	characteristic::{
		active::ActiveCharacteristic, active_identifier::ActiveIdentifierCharacteristic,
		button_event::ButtonEventCharacteristic, name::NameCharacteristic, HapCharacteristic,
	},
	service::HapService,
	HapType,
};

/// Target Control service.
#[derive(Debug, Default)]
pub struct TargetControlService {
	/// Instance ID of the Target Control service.
	id: u64,
	/// [`HapType`](HapType) of the Target Control service.
	hap_type: HapType,
	/// When set to true, this service is not visible to user.
	hidden: bool,
	/// When set to true, this is the primary service on the accessory.
	primary: bool,
	/// An array of numbers containing the instance IDs of the services that this service links to.
	linked_services: Vec<u64>,

	/// Active characteristic (required).
	pub active: ActiveCharacteristic,
	/// Active Identifier characteristic (required).
	pub active_identifier: ActiveIdentifierCharacteristic,
	/// Button Event characteristic (required).
	pub button_event: ButtonEventCharacteristic,

	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
}

impl TargetControlService {
	/// Creates a new Target Control service.
	pub fn new(id: u64, accessory_id: u64) -> Self {
		Self {
			id,
			hap_type: HapType::TargetControl,
			active: ActiveCharacteristic::new(id + 1, accessory_id),
			active_identifier: ActiveIdentifierCharacteristic::new(id + 1 + 1, accessory_id),
			button_event: ButtonEventCharacteristic::new(id + 1 + 2, accessory_id),
			name: Some(NameCharacteristic::new(id + 1 + 3, accessory_id)),
			..Default::default()
		}
	}
}

impl HapService for TargetControlService {
	fn get_id(&self) -> u64 {
		self.id
	}

	fn set_id(&mut self, id: u64) {
		self.id = id;
	}

	fn get_type(&self) -> HapType {
		self.hap_type
	}

	fn set_type(&mut self, hap_type: HapType) {
		self.hap_type = hap_type;
	}

	fn get_hidden(&self) -> bool {
		self.hidden
	}

	fn set_hidden(&mut self, hidden: bool) {
		self.hidden = hidden;
	}

	fn get_primary(&self) -> bool {
		self.primary
	}

	fn set_primary(&mut self, primary: bool) {
		self.primary = primary;
	}

	fn get_linked_services(&self) -> Vec<u64> {
		self.linked_services.clone()
	}

	fn set_linked_services(&mut self, linked_services: Vec<u64>) {
		self.linked_services = linked_services;
	}

	fn get_characteristic(&self, hap_type: HapType) -> Option<&dyn HapCharacteristic> {
		self.get_characteristics()
			.into_iter()
			.find(|&characteristic| characteristic.get_type() == hap_type)
	}

	fn get_mut_characteristic(&mut self, hap_type: HapType) -> Option<&mut dyn HapCharacteristic> {
		self.get_mut_characteristics()
			.into_iter()
			.find(|characteristic| characteristic.get_type() == hap_type)
	}

	fn get_characteristics(&self) -> Vec<&dyn HapCharacteristic> {
		#[allow(unused_mut)]
		let mut characteristics: Vec<&dyn HapCharacteristic> =
			vec![&self.active, &self.active_identifier, &self.button_event];
		if let Some(c) = &self.name {
			characteristics.push(c);
		}
		characteristics
	}

	fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
		#[allow(unused_mut)]
		let mut characteristics: Vec<&mut dyn HapCharacteristic> =
			vec![&mut self.active, &mut self.active_identifier, &mut self.button_event];
		if let Some(c) = &mut self.name {
			characteristics.push(c);
		}
		characteristics
	}
}

impl Serialize for TargetControlService {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		let mut state = serializer.serialize_struct("HapService", 5)?;
		state.serialize_field("iid", &self.get_id())?;
		state.serialize_field("type", &self.get_type())?;
		state.serialize_field("hidden", &self.get_hidden())?;
		state.serialize_field("primary", &self.get_primary())?;
		state.serialize_field("characteristics", &self.get_characteristics())?;
		// linked services left out for now
		state.end()
	}
}
