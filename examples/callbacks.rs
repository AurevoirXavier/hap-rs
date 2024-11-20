use hap::{
	accessory::{lightbulb::LightbulbAccessory, AccessoryCategory, AccessoryInformation},
	characteristic::CharacteristicCallbacks,
	server::{IpServer, Server},
	storage::{FileStorage, Storage},
	Config, MacAddress, Pin, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
	let mut lightbulb = LightbulbAccessory::new(
		1,
		AccessoryInformation { name: "Acme Lightbulb".into(), ..Default::default() },
	)?;

	lightbulb.lightbulb.power_state.on_read(Some(|| {
		println!("power_state characteristic read");
		Ok(None)
	}));
	lightbulb.lightbulb.power_state.on_update(Some(|current_val: &bool, new_val: &bool| {
		println!("power_state characteristic updated from {} to {}", current_val, new_val);
		Ok(())
	}));

	let mut storage = FileStorage::current_dir().await?;

	let config = match storage.load_config().await {
		Ok(mut config) => {
			config.redetermine_local_ip();
			storage.save_config(&config).await?;
			config
		},
		Err(_) => {
			let config = Config {
				pin: Pin::new([1, 1, 1, 2, 2, 3, 3, 3])?,
				name: "Acme Lightbulb".into(),
				device_id: MacAddress::from([10, 20, 30, 40, 50, 60]),
				category: AccessoryCategory::Lightbulb,
				..Default::default()
			};
			storage.save_config(&config).await?;
			config
		},
	};

	let server = IpServer::new(config, storage).await?;
	server.add_accessory(lightbulb).await?;

	let handle = server.run_handle();

	std::env::set_var("RUST_LOG", "hap=debug");
	env_logger::init();

	handle.await
}
