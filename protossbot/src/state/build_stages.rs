use rsbwapi::{UnitType, UpgradeType};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BuildStage {
  pub name: String,
  pub desired_counts: HashMap<UnitType, i32>,
  pub desired_upgrades: Vec<UpgradeType>,
}

impl BuildStage {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_string(),
      desired_counts: HashMap::new(),
      desired_upgrades: Vec::new(),
    }
  }

  pub fn with_unit(mut self, unit_type: UnitType, count: i32) -> Self {
    self.desired_counts.insert(unit_type, count);
    self
  }

  pub fn with_upgrade(mut self, upgrade_type: UpgradeType) -> Self {
    self.desired_upgrades.push(upgrade_type);
    self
  }
}

pub fn get_build_stages() -> Vec<BuildStage> {
  vec![
    BuildStage::new("Start")
      .with_unit(UnitType::Terran_SCV, 8)
      .with_unit(UnitType::Terran_Supply_Depot, 1)
      .with_unit(UnitType::Terran_Barracks, 1),
    BuildStage::new("Basic Production")
      .with_unit(UnitType::Terran_Refinery, 1)
      .with_unit(UnitType::Terran_SCV, 14),
    BuildStage::new("stage 3")
      .with_unit(UnitType::Terran_SCV, 24)
      .with_unit(UnitType::Terran_Factory, 1)
      .with_unit(UnitType::Terran_Machine_Shop, 1)
      .with_unit(UnitType::Terran_Supply_Depot, 2),
    // .with_unit(UnitType::Terran_Marine, 4)
    // .with_unit(UnitType::Terran_Starport, 1),
    // BuildStage::new("Mid Game")
    //   .with_unit(UnitType::Terran_SCV, 28)
    //   // .with_unit(UnitType::Terran_Engineering_Bay, 1)
    //   .with_unit(UnitType::Terran_Supply_Depot, 2)
    //   .with_unit(UnitType::Terran_Science_Facility, 1)
    //   .with_unit(UnitType::Terran_Physics_Lab, 1)
    //   .with_unit(UnitType::Terran_Armory, 1)
    //   .with_unit(UnitType::Terran_Command_Center, 2)
    //   .with_unit(UnitType::Terran_Wraith, 1)
    //   // .with_upgrade(UpgradeType::Terran_Infantry_Weapons)
    //   .with_unit(UnitType::Terran_Marine, 7),
    // BuildStage::new("next step")
    //   .with_unit(UnitType::Terran_Starport, 3)
    //   .with_unit(UnitType::Terran_Control_Tower, 3)
    //   // .with_upgrade(UpgradeType::U_238_Shells)
    //   // .with_upgrade(UpgradeType::Terran_Infantry_Armor)
    //   // .with_unit(UnitType::Terran_Missile_Turret, 1)
    //   .with_unit(UnitType::Terran_Battlecruiser, 1)
    //   .with_unit(UnitType::Terran_Marine, 10)
    //   .with_unit(UnitType::Terran_SCV, 30),
    // BuildStage::new("late game tech")
    //   .with_unit(UnitType::Terran_SCV, 30)
    //   .with_unit(UnitType::Terran_Factory, 2)
    //   .with_unit(UnitType::Terran_Armory, 1)
  ]
}
