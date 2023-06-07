//use std::fs::File;
//use std::io::prelude::Write;
use std::collections::BTreeMap;
use std::println;

use serde::{Deserialize, Serialize};

enum LootboxType {
    Standard, // Standard Lootbox requires a single click to open.
    Teaser,   // Teaser Lootbox requires 2 clicks, and will show a teaser of the contents.
}

struct PanelBoilerplate {
    panel_name: String, // Panel Internal Name. Used in /cpanel command EX: /cpanel panel_name
    lootbox_type: LootboxType, //The type of lootbox.
    chance_command: String, // Chance Command.
    perm: String,
    rows: String,
    title: String, // Panel Display Name
    commands_on_open: Vec<String>,
    panel_type: Vec<String>,
    item_details: ItemDetails,
}

struct ItemDetails {
    lootbox_slot: String, // Slot Lootbox is located on the command panel.
    material: String,
    name: String,
    commands: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Panels {
    panels: BTreeMap<String, PanelConfig>,
}

/*
panels:
  [PanelConfig]:
    item:
      [itemsettings]:
        has:
          [hassection]:
*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PanelConfig {
    perm: String,
    rows: String,
    title: String,

    #[serde(
        rename(serialize = "commands-on-open"),
        skip_serializing_if = "Vec::is_empty"
    )]
    commands_on_open: Vec<String>,

    #[serde(rename(serialize = "panelType"))]
    panel_type: Vec<String>,

    item: BTreeMap<String, ItemSettings>,
}

/*
[panels]:
  panelconfig:
    perm: "String"
    rows: "String"
    title: "String"
    commands-on-open:
    - "String"
    - "String"
    panelType:
    - "String"
    - "String"
    item:
      [itemsettings]:
        has:
          [hassection]:
*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ItemSettings {
    material: String,
    name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    commands: Vec<String>,

    #[serde(flatten)]
    has: BTreeMap<String, HasSections>,
}

/*
[panels]:
  [panelsconfig]:
    item:
      itemsettings:
        material: "String"
        name: "String"
        commands:
        - "String"
        has:
          [hassection]:
*/

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct HasSections {
    value0: u32,
    compare0: String,
    material: String,
    stack: u32,
    name: String,
    commands: Vec<String>,
}

/*
[panels]:
  [panelsconfig]:
    [item]:
      [itemsettings]:
        has:
          value0: String
          compare0: String
          material: String
          stack: String
          name: String
          commands:
          - String

*/

fn main() -> Result<(), serde_yaml::Error> {
    let item_details = ItemDetails {
        lootbox_slot: "4".to_string(),
        material: "STONE".to_string(),
        name: "&fClick to Open!".to_string(),
        commands: Vec::new(),
    };

    let mut panel_plate = PanelBoilerplate {
        panel_name: "rustybox".to_string(),
        lootbox_type: LootboxType::Teaser,
        chance_command: "set-data= chance %cp-random-1,133%".to_string(),

        perm: "default".to_string(),
        rows: "DROPPER".to_string(),
        title: "&8Rusty Lootbox".to_string(),
        commands_on_open: Vec::new(),
        panel_type: vec!["unclosable".to_string(), "nocommand".to_string()],
        item_details,
    };
    match panel_plate.lootbox_type {
        LootboxType::Standard => {
            panel_plate.commands_on_open = vec![panel_plate.chance_command.clone()];
        }
        LootboxType::Teaser => {
            panel_plate.item_details.commands = vec![panel_plate.chance_command.clone()];
        }
    }

    let panels = main_panel(panel_plate);

    let yaml = serde_yaml::to_string(&panels)?;
    println!("{}", yaml);
    Ok(())
}

fn main_panel(boiler_plate: PanelBoilerplate) -> Panels {
    Panels {
        panels: main_panel_map(boiler_plate),
    }
}

fn main_panel_map(boiler_plate: PanelBoilerplate) -> BTreeMap<String, PanelConfig> {
    let mut main_panel = BTreeMap::new();
    main_panel.insert(boiler_plate.panel_name.clone(), panel_config(boiler_plate));
    main_panel
}

fn panel_config(boiler_plate: PanelBoilerplate) -> PanelConfig {
    PanelConfig {
        perm: boiler_plate.perm,
        rows: boiler_plate.rows,
        title: boiler_plate.title,
        commands_on_open: boiler_plate.commands_on_open,
        panel_type: boiler_plate.panel_type,
        item: item_settings_map(boiler_plate.item_details),
    }
}

fn item_settings_map(item_details: ItemDetails) -> BTreeMap<String, ItemSettings> {
    let mut item_settings_map = BTreeMap::new();
    item_settings_map.insert(
        item_details.lootbox_slot.clone(),
        item_settings(item_details),
    );
    item_settings_map
}

fn item_settings(item_details: ItemDetails) -> ItemSettings {
    ItemSettings {
        material: item_details.material,
        name: item_details.name,
        commands: item_details.commands,
        has: has_section_map(),
    }
}

fn has_section_map() -> BTreeMap<String, HasSections> {
    let mut has_section_map = BTreeMap::new();
    has_section_map.insert(String::from("has0"), has_section());
    has_section_map
}

fn has_section() -> HasSections {
    HasSections {
        value0: 1,
        compare0: String::from("%cp-data-chance%"),
        material: String::from("Stone"),
        stack: 1,
        name: String::from("&fClick to get!"),
        commands: vec![
            String::from("set-data chance 0"),
            String::from("msg- &fYou got $100"),
            String::from("console= cmi money give %cp-player-name% 100"),
            String::from("cpc"),
        ],
    }
}
