use tinybmp::Bmp;

#[cfg(feature = "icons16x16")]
pub fn wi_alien_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-alien_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_alien_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-alien_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_alien_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-alien_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_alien_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-alien_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_barometer_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-barometer_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_barometer_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-barometer_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_barometer_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-barometer_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_barometer_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-barometer_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_celsius_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-celsius_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_celsius_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-celsius_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_celsius_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-celsius_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_celsius_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-celsius_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloud_down_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-down_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloud_down_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-down_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloud_down_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-down_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloud_down_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-down_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloud_refresh_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-refresh_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloud_refresh_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-refresh_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloud_refresh_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-refresh_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloud_refresh_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-refresh_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloud_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloud_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloud_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloud_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloud_up_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-up_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloud_up_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-up_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloud_up_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-up_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloud_up_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloud-up_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloudy_gusts_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-gusts_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloudy_gusts_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-gusts_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloudy_gusts_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-gusts_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloudy_gusts_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-gusts_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_cloudy_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_cloudy_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_cloudy_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_cloudy_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-cloudy-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_cloudy_gusts_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-gusts_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_cloudy_gusts_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-gusts_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_cloudy_gusts_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-gusts_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_cloudy_gusts_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-gusts_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_cloudy_high_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-high_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_cloudy_high_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-high_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_cloudy_high_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-high_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_cloudy_high_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-high_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_cloudy_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_cloudy_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_cloudy_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_cloudy_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-cloudy-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_fog_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-fog_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_fog_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-fog_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_fog_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-fog_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_fog_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-fog_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_hail_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-hail_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_hail_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-hail_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_hail_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-hail_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_hail_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-hail_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_haze_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-haze_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_haze_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-haze_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_haze_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-haze_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_haze_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-haze_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_lightning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-lightning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_lightning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-lightning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_lightning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-lightning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_lightning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-lightning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_light_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-light-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_light_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-light-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_light_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-light-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_light_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-light-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_rain_mix_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-mix_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_rain_mix_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-mix_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_rain_mix_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-mix_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_rain_mix_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-mix_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_rain_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_rain_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_rain_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_rain_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_rain_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_rain_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_rain_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_rain_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-rain-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_sleet_storm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet-storm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_sleet_storm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet-storm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_sleet_storm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet-storm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_sleet_storm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet-storm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_sleet_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_sleet_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_sleet_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_sleet_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sleet_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_snow_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_snow_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_snow_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_snow_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_snow_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_snow_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_snow_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_snow_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_snow_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_snow_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_snow_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_snow_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-snow-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_sprinkle_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sprinkle_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_sprinkle_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sprinkle_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_sprinkle_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sprinkle_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_sprinkle_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sprinkle_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_storm_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-storm-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_storm_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-storm-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_storm_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-storm-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_storm_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-storm-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_sunny_overcast_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny-overcast_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_sunny_overcast_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny-overcast_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_sunny_overcast_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny-overcast_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_sunny_overcast_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny-overcast_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_sunny_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_sunny_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_sunny_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_sunny_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-sunny_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_day_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_day_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_day_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_day_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-day-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_degrees_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-degrees_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_degrees_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-degrees_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_degrees_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-degrees_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_degrees_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-degrees_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_down_left_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-left_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_down_left_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-left_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_down_left_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-left_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_down_left_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-left_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_down_right_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-right_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_down_right_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-right_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_down_right_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-right_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_down_right_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down-right_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_down_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_down_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_down_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_down_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-down_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_left_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-left_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_left_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-left_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_left_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-left_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_left_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-left_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_right_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-right_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_right_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-right_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_right_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-right_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_right_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-right_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_up_left_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-left_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_up_left_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-left_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_up_left_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-left_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_up_left_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-left_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_up_right_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-right_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_up_right_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-right_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_up_right_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-right_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_up_right_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up-right_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_direction_up_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_direction_up_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_direction_up_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_direction_up_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-direction-up_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_dust_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-dust_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_dust_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-dust_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_dust_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-dust_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_dust_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-dust_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_earthquake_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-earthquake_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_earthquake_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-earthquake_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_earthquake_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-earthquake_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_earthquake_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-earthquake_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_fahrenheit_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fahrenheit_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_fahrenheit_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fahrenheit_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_fahrenheit_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fahrenheit_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_fahrenheit_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fahrenheit_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_fire_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fire_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_fire_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fire_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_fire_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fire_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_fire_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fire_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_flood_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-flood_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_flood_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-flood_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_flood_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-flood_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_flood_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-flood_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_fog_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fog_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_fog_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fog_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_fog_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fog_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_fog_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-fog_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_gale_warning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-gale-warning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_gale_warning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-gale-warning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_gale_warning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-gale-warning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_gale_warning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-gale-warning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_hail_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hail_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_hail_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hail_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_hail_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hail_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_hail_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hail_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_horizon_alt_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon-alt_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_horizon_alt_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon-alt_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_horizon_alt_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon-alt_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_horizon_alt_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon-alt_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_horizon_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_horizon_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_horizon_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_horizon_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-horizon_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_hot_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hot_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_hot_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hot_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_hot_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hot_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_hot_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hot_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_humidity_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-humidity_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_humidity_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-humidity_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_humidity_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-humidity_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_humidity_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-humidity_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_hurricane_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_hurricane_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_hurricane_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_hurricane_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_hurricane_warning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane-warning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_hurricane_warning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane-warning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_hurricane_warning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane-warning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_hurricane_warning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-hurricane-warning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_lightning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lightning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_lightning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lightning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_lightning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lightning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_lightning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lightning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_lunar_eclipse_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lunar-eclipse_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_lunar_eclipse_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lunar-eclipse_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_lunar_eclipse_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lunar-eclipse_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_lunar_eclipse_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-lunar-eclipse_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_meteor_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-meteor_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_meteor_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-meteor_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_meteor_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-meteor_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_meteor_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-meteor_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_first_quarter_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-first-quarter_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_first_quarter_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-first-quarter_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_first_quarter_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-first-quarter_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_first_quarter_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-first-quarter_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_full_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-full_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_full_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-full_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_full_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-full_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_full_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-full_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_new_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-new_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_new_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-new_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_new_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-new_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_new_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-new_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_third_quarter_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-third-quarter_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_third_quarter_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-third-quarter_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_third_quarter_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-third-quarter_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_third_quarter_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-third-quarter_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_crescent_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_crescent_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_crescent_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_crescent_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-crescent-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waning_gibbous_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waning_gibbous_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waning_gibbous_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waning_gibbous_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waning-gibbous-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_crescent_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_crescent_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_crescent_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_crescent_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-crescent-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_alt_waxing_gibbous_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_alt_waxing_gibbous_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_alt_waxing_gibbous_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_alt_waxing_gibbous_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-alt-waxing-gibbous-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_first_quarter_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-first-quarter_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_first_quarter_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-first-quarter_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_first_quarter_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-first-quarter_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_first_quarter_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-first-quarter_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_full_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-full_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_full_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-full_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_full_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-full_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_full_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-full_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_new_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-new_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_new_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-new_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_new_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-new_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_new_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-new_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moonrise_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonrise_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moonrise_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonrise_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moonrise_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonrise_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moonrise_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonrise_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moonset_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonset_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moonset_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonset_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moonset_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonset_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moonset_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moonset_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_third_quarter_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-third-quarter_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_third_quarter_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-third-quarter_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_third_quarter_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-third-quarter_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_third_quarter_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-third-quarter_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_crescent_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_crescent_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_crescent_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_crescent_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-crescent-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waning_gibbous_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waning_gibbous_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waning_gibbous_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waning_gibbous_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waning-gibbous-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_crescent_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_crescent_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_crescent_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_crescent_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-crescent-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_moon_waxing_gibbous_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_moon_waxing_gibbous_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_moon_waxing_gibbous_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_moon_waxing_gibbous_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-moon-waxing-gibbous-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_na_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-na_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_na_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-na_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_na_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-na_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_na_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-na_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_cloudy_gusts_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-gusts_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_cloudy_gusts_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-gusts_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_cloudy_gusts_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-gusts_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_cloudy_gusts_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-gusts_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_cloudy_high_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-high_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_cloudy_high_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-high_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_cloudy_high_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-high_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_cloudy_high_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-high_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_cloudy_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_cloudy_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_cloudy_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_cloudy_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-cloudy-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_hail_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-hail_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_hail_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-hail_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_hail_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-hail_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_hail_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-hail_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_lightning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-lightning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_lightning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-lightning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_lightning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-lightning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_lightning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-lightning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_partly_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-partly-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_partly_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-partly-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_partly_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-partly-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_partly_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-partly-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_rain_mix_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-mix_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_rain_mix_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-mix_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_rain_mix_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-mix_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_rain_mix_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-mix_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_rain_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_rain_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_rain_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_rain_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_rain_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_rain_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_rain_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_rain_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-rain-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_sleet_storm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet-storm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_sleet_storm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet-storm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_sleet_storm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet-storm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_sleet_storm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet-storm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_sleet_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_sleet_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_sleet_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_sleet_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sleet_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_snow_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_snow_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_snow_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_snow_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_snow_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_snow_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_snow_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_snow_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_snow_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_snow_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_snow_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_snow_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-snow-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_sprinkle_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sprinkle_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_sprinkle_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sprinkle_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_sprinkle_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sprinkle_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_sprinkle_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-sprinkle_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_storm_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-storm-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_storm_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-storm-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_storm_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-storm-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_storm_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-storm-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_alt_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_alt_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_alt_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_alt_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-alt-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_clear_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-clear_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_clear_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-clear_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_clear_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-clear_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_clear_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-clear_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_cloudy_gusts_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-gusts_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_cloudy_gusts_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-gusts_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_cloudy_gusts_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-gusts_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_cloudy_gusts_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-gusts_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_cloudy_high_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-high_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_cloudy_high_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-high_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_cloudy_high_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-high_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_cloudy_high_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-high_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_cloudy_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_cloudy_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_cloudy_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_cloudy_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-cloudy-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_fog_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-fog_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_fog_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-fog_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_fog_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-fog_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_fog_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-fog_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_hail_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-hail_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_hail_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-hail_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_hail_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-hail_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_hail_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-hail_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_lightning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-lightning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_lightning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-lightning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_lightning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-lightning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_lightning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-lightning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_partly_cloudy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-partly-cloudy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_partly_cloudy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-partly-cloudy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_partly_cloudy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-partly-cloudy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_partly_cloudy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-partly-cloudy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_rain_mix_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-mix_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_rain_mix_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-mix_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_rain_mix_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-mix_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_rain_mix_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-mix_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_rain_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_rain_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_rain_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_rain_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_rain_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_rain_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_rain_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_rain_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-rain-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_sleet_storm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet-storm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_sleet_storm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet-storm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_sleet_storm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet-storm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_sleet_storm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet-storm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_sleet_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_sleet_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_sleet_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_sleet_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sleet_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_snow_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_snow_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_snow_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_snow_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_snow_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_snow_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_snow_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_snow_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_snow_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_snow_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_snow_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_snow_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-snow-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_sprinkle_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sprinkle_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_sprinkle_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sprinkle_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_sprinkle_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sprinkle_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_sprinkle_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-sprinkle_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_storm_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-storm-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_storm_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-storm-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_storm_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-storm-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_storm_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-storm-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_night_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_night_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_night_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_night_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-night-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_raindrops_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrops_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_raindrops_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrops_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_raindrops_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrops_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_raindrops_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrops_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_raindrop_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrop_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_raindrop_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrop_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_raindrop_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrop_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_raindrop_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-raindrop_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_rain_mix_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-mix_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_rain_mix_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-mix_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_rain_mix_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-mix_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_rain_mix_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-mix_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_rain_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_rain_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_rain_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_rain_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_rain_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_rain_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_rain_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_rain_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-rain-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_refresh_alt_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh-alt_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_refresh_alt_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh-alt_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_refresh_alt_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh-alt_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_refresh_alt_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh-alt_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_refresh_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_refresh_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_refresh_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_refresh_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-refresh_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_sandstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sandstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_sandstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sandstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_sandstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sandstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_sandstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sandstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_sleet_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sleet_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_sleet_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sleet_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_sleet_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sleet_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_sleet_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sleet_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_small_craft_advisory_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-small-craft-advisory_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_small_craft_advisory_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-small-craft-advisory_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_small_craft_advisory_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-small-craft-advisory_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_small_craft_advisory_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-small-craft-advisory_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_smog_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smog_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_smog_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smog_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_smog_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smog_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_smog_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smog_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_smoke_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smoke_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_smoke_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smoke_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_smoke_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smoke_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_smoke_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-smoke_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_snowflake_cold_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snowflake-cold_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_snowflake_cold_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snowflake-cold_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_snowflake_cold_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snowflake-cold_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_snowflake_cold_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snowflake-cold_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_snow_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_snow_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_snow_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_snow_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_snow_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_snow_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_snow_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_snow_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-snow-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_solar_eclipse_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-solar-eclipse_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_solar_eclipse_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-solar-eclipse_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_solar_eclipse_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-solar-eclipse_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_solar_eclipse_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-solar-eclipse_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_sprinkle_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sprinkle_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_sprinkle_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sprinkle_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_sprinkle_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sprinkle_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_sprinkle_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sprinkle_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_stars_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-stars_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_stars_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-stars_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_stars_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-stars_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_stars_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-stars_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_storm_showers_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-showers_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_storm_showers_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-showers_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_storm_showers_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-showers_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_storm_showers_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-showers_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_storm_warning_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-warning_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_storm_warning_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-warning_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_storm_warning_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-warning_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_storm_warning_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-storm-warning_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_strong_wind_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-strong-wind_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_strong_wind_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-strong-wind_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_strong_wind_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-strong-wind_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_strong_wind_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-strong-wind_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_sunrise_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunrise_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_sunrise_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunrise_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_sunrise_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunrise_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_sunrise_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunrise_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_sunset_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunset_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_sunset_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunset_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_sunset_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunset_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_sunset_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-sunset_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_thermometer_exterior_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-exterior_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_thermometer_exterior_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-exterior_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_thermometer_exterior_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-exterior_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_thermometer_exterior_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-exterior_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_thermometer_internal_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-internal_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_thermometer_internal_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-internal_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_thermometer_internal_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-internal_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_thermometer_internal_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer-internal_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_thermometer_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_thermometer_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_thermometer_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_thermometer_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thermometer_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_thunderstorm_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thunderstorm_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_thunderstorm_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thunderstorm_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_thunderstorm_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thunderstorm_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_thunderstorm_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-thunderstorm_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_10_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-10_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_10_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-10_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_10_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-10_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_10_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-10_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_11_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-11_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_11_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-11_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_11_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-11_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_11_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-11_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_12_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-12_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_12_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-12_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_12_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-12_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_12_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-12_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_7_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-7_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_7_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-7_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_7_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-7_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_7_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-7_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_8_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-8_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_8_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-8_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_8_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-8_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_8_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-8_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_time_9_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-9_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_time_9_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-9_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_time_9_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-9_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_time_9_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-time-9_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_tornado_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tornado_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_tornado_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tornado_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_tornado_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tornado_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_tornado_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tornado_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_train_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-train_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_train_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-train_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_train_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-train_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_train_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-train_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_tsunami_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tsunami_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_tsunami_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tsunami_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_tsunami_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tsunami_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_tsunami_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-tsunami_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_umbrella_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-umbrella_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_umbrella_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-umbrella_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_umbrella_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-umbrella_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_umbrella_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-umbrella_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_volcano_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-volcano_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_volcano_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-volcano_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_volcano_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-volcano_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_volcano_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-volcano_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_0_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-0_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_0_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-0_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_0_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-0_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_0_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-0_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_10_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-10_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_10_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-10_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_10_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-10_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_10_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-10_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_11_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-11_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_11_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-11_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_11_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-11_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_11_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-11_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_12_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-12_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_12_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-12_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_12_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-12_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_12_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-12_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_1_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-1_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_1_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-1_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_1_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-1_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_1_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-1_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_2_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-2_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_2_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-2_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_2_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-2_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_2_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-2_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_3_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-3_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_3_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-3_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_3_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-3_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_3_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-3_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_4_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-4_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_4_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-4_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_4_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-4_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_4_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-4_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_5_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-5_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_5_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-5_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_5_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-5_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_5_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-5_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_6_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-6_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_6_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-6_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_6_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-6_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_6_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-6_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_7_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-7_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_7_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-7_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_7_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-7_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_7_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-7_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_8_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-8_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_8_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-8_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_8_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-8_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_8_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-8_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_beaufort_9_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-9_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_beaufort_9_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-9_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_beaufort_9_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-9_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_beaufort_9_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-beaufort-9_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_wind_deg_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-deg_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_wind_deg_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-deg_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_wind_deg_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-deg_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_wind_deg_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-wind-deg_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons16x16")]
pub fn wi_windy_16x16() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-windy_16x16.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons32x32")]
pub fn wi_windy_32x32() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-windy_32x32.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons64x64")]
pub fn wi_windy_64x64() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-windy_64x64.bmp"));
  Bmp::from_slice(IMAGE)
}

#[cfg(feature = "icons128x128")]
pub fn wi_windy_128x128() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/wi-windy_128x128.bmp"));
  Bmp::from_slice(IMAGE)
}
