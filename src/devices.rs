/// Implemented by all Tapo devices.
pub trait TapoDeviceExt: std::fmt::Debug {}

/// Basic functionality of all Tapo [devices](https://www.tapo.com/en/).
#[derive(Debug)]
pub struct GenericDevice;
impl TapoDeviceExt for GenericDevice {}

/// Tapo [L510](https://www.tapo.com/en/search/?q=L510) devices.
#[derive(Debug)]
pub struct L510;
impl TapoDeviceExt for L510 {}

/// Tapo [L530](https://www.tapo.com/en/search/?q=L530) devices.
#[derive(Debug)]
pub struct L530;
impl TapoDeviceExt for L530 {}

/// Tapo [P100](https://www.tapo.com/en/search/?q=P100) & [P105](https://www.tapo.com/en/search/?q=P105) devices.
#[derive(Debug)]
pub struct P100;
impl TapoDeviceExt for P100 {}

/// Tapo [P110](https://www.tapo.com/en/search/?q=P110) & P115](https://www.tapo.com/en/search/?q=P115) devices.
#[derive(Debug)]
pub struct P110;
impl TapoDeviceExt for P110 {}
