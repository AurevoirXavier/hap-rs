// this file is auto-generated by hap-codegen

use serde::ser::{Serialize, SerializeStruct, Serializer};

use crate::{
	characteristic::{
		active::ActiveCharacteristic, active_identifier::ActiveIdentifierCharacteristic,
		brightness::BrightnessCharacteristic, closed_captions::ClosedCaptionsCharacteristic,
		configured_name::ConfiguredNameCharacteristic,
		current_media_state::CurrentMediaStateCharacteristic,
		display_order::DisplayOrderCharacteristic, name::NameCharacteristic,
		picture_mode::PictureModeCharacteristic,
		power_mode_selection::PowerModeSelectionCharacteristic,
		remote_key::RemoteKeyCharacteristic,
		sleep_discovery_mode::SleepDiscoveryModeCharacteristic,
		target_media_state::TargetMediaStateCharacteristic, HapCharacteristic,
	},
	service::HapService,
	HapType,
};

/// Television service.
#[derive(Debug, Default)]
pub struct TelevisionService {
	/// Instance ID of the Television service.
	id: u64,
	/// [`HapType`](HapType) of the Television service.
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
	/// Configured Name characteristic (required).
	pub configured_name: ConfiguredNameCharacteristic,
	/// Remote Key characteristic (required).
	pub remote_key: RemoteKeyCharacteristic,
	/// Sleep Discovery Mode characteristic (required).
	pub sleep_discovery_mode: SleepDiscoveryModeCharacteristic,

	/// Brightness characteristic (optional).
	pub brightness: Option<BrightnessCharacteristic>,
	/// Closed Captions characteristic (optional).
	pub closed_captions: Option<ClosedCaptionsCharacteristic>,
	/// Display Order characteristic (optional).
	pub display_order: Option<DisplayOrderCharacteristic>,
	/// Current Media State characteristic (optional).
	pub current_media_state: Option<CurrentMediaStateCharacteristic>,
	/// Target Media State characteristic (optional).
	pub target_media_state: Option<TargetMediaStateCharacteristic>,
	/// Name characteristic (optional).
	pub name: Option<NameCharacteristic>,
	/// Picture Mode characteristic (optional).
	pub picture_mode: Option<PictureModeCharacteristic>,
	/// Power Mode Selection characteristic (optional).
	pub power_mode_selection: Option<PowerModeSelectionCharacteristic>,
}

impl TelevisionService {
	/// Creates a new Television service.
	pub fn new(id: u64, accessory_id: u64) -> Self {
		Self {
			id,
			hap_type: HapType::Television,
			active: ActiveCharacteristic::new(id + 1, accessory_id),
			active_identifier: ActiveIdentifierCharacteristic::new(id + 1 + 1, accessory_id),
			configured_name: ConfiguredNameCharacteristic::new(id + 1 + 2, accessory_id),
			remote_key: RemoteKeyCharacteristic::new(id + 1 + 3, accessory_id),
			sleep_discovery_mode: SleepDiscoveryModeCharacteristic::new(id + 1 + 4, accessory_id),
			brightness: Some(BrightnessCharacteristic::new(id + 1 + 5, accessory_id)),
			closed_captions: Some(ClosedCaptionsCharacteristic::new(id + 1 + 1 + 5, accessory_id)),
			display_order: Some(DisplayOrderCharacteristic::new(id + 1 + 2 + 5, accessory_id)),
			current_media_state: Some(CurrentMediaStateCharacteristic::new(
				id + 1 + 3 + 5,
				accessory_id,
			)),
			target_media_state: Some(TargetMediaStateCharacteristic::new(
				id + 1 + 4 + 5,
				accessory_id,
			)),
			name: Some(NameCharacteristic::new(id + 1 + 5 + 5, accessory_id)),
			picture_mode: Some(PictureModeCharacteristic::new(id + 1 + 6 + 5, accessory_id)),
			power_mode_selection: Some(PowerModeSelectionCharacteristic::new(
				id + 1 + 7 + 5,
				accessory_id,
			)),
			..Default::default()
		}
	}
}

impl HapService for TelevisionService {
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
		let mut characteristics: Vec<&dyn HapCharacteristic> = vec![
			&self.active,
			&self.active_identifier,
			&self.configured_name,
			&self.remote_key,
			&self.sleep_discovery_mode,
		];
		if let Some(c) = &self.brightness {
			characteristics.push(c);
		}
		if let Some(c) = &self.closed_captions {
			characteristics.push(c);
		}
		if let Some(c) = &self.display_order {
			characteristics.push(c);
		}
		if let Some(c) = &self.current_media_state {
			characteristics.push(c);
		}
		if let Some(c) = &self.target_media_state {
			characteristics.push(c);
		}
		if let Some(c) = &self.name {
			characteristics.push(c);
		}
		if let Some(c) = &self.picture_mode {
			characteristics.push(c);
		}
		if let Some(c) = &self.power_mode_selection {
			characteristics.push(c);
		}
		characteristics
	}

	fn get_mut_characteristics(&mut self) -> Vec<&mut dyn HapCharacteristic> {
		#[allow(unused_mut)]
		let mut characteristics: Vec<&mut dyn HapCharacteristic> = vec![
			&mut self.active,
			&mut self.active_identifier,
			&mut self.configured_name,
			&mut self.remote_key,
			&mut self.sleep_discovery_mode,
		];
		if let Some(c) = &mut self.brightness {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.closed_captions {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.display_order {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.current_media_state {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.target_media_state {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.name {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.picture_mode {
			characteristics.push(c);
		}
		if let Some(c) = &mut self.power_mode_selection {
			characteristics.push(c);
		}
		characteristics
	}
}

impl Serialize for TelevisionService {
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
