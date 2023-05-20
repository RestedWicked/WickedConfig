//use std::fs::File;
//use std::io::prelude::Write;
use std::collections::BTreeMap;
use std::println;

use serde::{Deserialize, Serialize};

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
    value0: String,
    compare0: String,
    material: String,
    stack: String,
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
    let panels = main_panel(String::from("rustybox"));

    let yaml = serde_yaml::to_string(&panels)?;
    println!("{}", yaml);
    Ok(())
}

fn main_panel(panel_name: String) -> Panels {
    Panels {
        panels: main_panel_map(panel_name),
    }
}

fn main_panel_map(panel_name: String) -> BTreeMap<String, PanelConfig> {
    let mut main_panel = BTreeMap::new();
    main_panel.insert(panel_name, panel_config());
    main_panel
}

fn panel_config() -> PanelConfig {
    PanelConfig {
        perm: String::from("default"),
        rows: String::from("DROPPER"),
        title: String::from("&8Rusty Lootbox"),
        commands_on_open: vec![String::from(
            "console= bpa progress quest %cp-player-name% 2 1 1",
        )],
        panel_type: vec![String::from("unclosable"), String::from("nocommand")],
        item: item_settings_map(),
    }
}

fn item_settings_map() -> BTreeMap<String, ItemSettings> {
    let mut item_settings_map = BTreeMap::new();
    item_settings_map.insert(String::from('4'), item_settings());
    item_settings_map
}

fn item_settings() -> ItemSettings {
    ItemSettings {
        material: String::from("STONE"),
        name: String::from("&fClick to Open!"),
        commands: vec![String::from("set-data= chance %cp-random-1,133%")],
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
        value0: String::from("1"),
        compare0: String::from("%cp-data-chance%"),
        material: String::from("Stone"),
        stack: String::from("1"),
        name: String::from("&fClick to get!"),
        commands: vec![
            String::from("set-data chance 0"),
            String::from("msg- &fYou got $100"),
            String::from("console= cmi money give %cp-player-name% 100"),
            String::from("cpc"),
        ],
    }
}
