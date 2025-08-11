use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod parse;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Chara {
    #[serde(rename = "MIK")]
    #[serde(alias = "MIKU")]
    Miku,
    #[serde(rename = "RIN")]
    Rin,
    #[serde(rename = "LEN")]
    Len,
    #[serde(rename = "LUK")]
    #[serde(alias = "LUKA")]
    Luka,
    #[serde(rename = "NER")]
    #[serde(alias = "NERU")]
    Neru,
    #[serde(rename = "HAK")]
    #[serde(alias = "HAKU")]
    Haku,
    #[serde(rename = "KAI")]
    #[serde(alias = "KAITO")]
    Kaito,
    #[serde(rename = "MEI")]
    #[serde(alias = "MEIKO")]
    Meiko,
    #[serde(rename = "SAK")]
    #[serde(alias = "SAKINE")]
    Sakine,
    #[serde(rename = "TET")]
    #[serde(alias = "TETO")]
    Teto,
    #[serde(rename = "EXT")]
    #[serde(alias = "EXTRA")]
    Extra,
    #[serde(rename = "ALL")]
    All,
}

impl ToString for Chara {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Miku => "Miku",
            Self::Rin => "Rin",
            Self::Len => "Len",
            Self::Luka => "Luka",
            Self::Neru => "Neru",
            Self::Haku => "Haku",
            Self::Kaito => "Kaito",
            Self::Meiko => "Meiko",
            Self::Sakine => "Sakine",
            Self::Teto => "Teto",
            Self::Extra => "Extra",
            Self::All => "All",
        })
    }
}

impl TryFrom<i32> for Chara {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Miku,
            1 => Self::Rin,
            2 => Self::Len,
            3 => Self::Luka,
            4 => Self::Neru,
            5 => Self::Haku,
            6 => Self::Kaito,
            7 => Self::Meiko,
            8 => Self::Sakine,
            9 => Self::Teto,
            10 => Self::Extra,
            11 => Self::All,
            _ => return Err(format!("Invalid value for Chara: {}", value)),
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemPart {
    Kami,
    Face,
    Neck,
    Zujo,
    Back,
}

impl ToString for ItemPart {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Kami => "Hair (Kami)",
            Self::Face => "Face",
            Self::Neck => "Neck",
            Self::Zujo => "Hat (Zujo)",
            Self::Back => "Back",
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i32)]
pub enum ItemSub {
    Zujo = 0,
    Kami = 1,
    Hitai = 2,
    Me = 3,
    Megane = 4,
    Mimi = 5,
    Kuchi = 6,
    Maki = 7,
    Kubi = 8,
    Inner = 9,
    Outer = 10,
    Kata = 11,
    UUde = 12,
    LUde = 13,
    Te = 14,
    JohaMae = 15,
    JohaUshiro = 16,
    Belt = 17,
    Kosi = 18,
    Pants = 19,
    Asi = 20,
    Sune = 21,
    Kutsu = 22,
    Hada = 23,
    Head = 24,
}

impl Into<i32> for ItemSub {
    fn into(self) -> i32 {
        match self {
            ItemSub::Zujo => 0,
            ItemSub::Kami => 1,
            ItemSub::Hitai => 2,
            ItemSub::Me => 3,
            ItemSub::Megane => 4,
            ItemSub::Mimi => 5,
            ItemSub::Kuchi => 6,
            ItemSub::Maki => 7,
            ItemSub::Kubi => 8,
            ItemSub::Inner => 9,
            ItemSub::Outer => 10,
            ItemSub::Kata => 11,
            ItemSub::UUde => 12,
            ItemSub::LUde => 13,
            ItemSub::Te => 14,
            ItemSub::JohaMae => 15,
            ItemSub::JohaUshiro => 16,
            ItemSub::Belt => 17,
            ItemSub::Kosi => 18,
            ItemSub::Pants => 19,
            ItemSub::Asi => 20,
            ItemSub::Sune => 21,
            ItemSub::Kutsu => 22,
            ItemSub::Hada => 23,
            ItemSub::Head => 24,
        }
    }
}

impl TryFrom<i32> for ItemSub {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::Zujo,
            1 => Self::Kami,
            2 => Self::Hitai,
            3 => Self::Me,
            4 => Self::Megane,
            5 => Self::Mimi,
            6 => Self::Kuchi,
            7 => Self::Maki,
            8 => Self::Kubi,
            9 => Self::Inner,
            10 => Self::Outer,
            11 => Self::Kata,
            12 => Self::UUde,
            13 => Self::LUde,
            14 => Self::Te,
            15 => Self::JohaMae,
            16 => Self::JohaUshiro,
            17 => Self::Belt,
            18 => Self::Kosi,
            19 => Self::Pants,
            20 => Self::Asi,
            21 => Self::Sune,
            22 => Self::Kutsu,
            23 => Self::Hada,
            24 => Self::Head,
            _ => return Err(format!("Invalid value for ItemSub: {}", value)),
        })
    }
}

impl ToString for ItemSub {
    fn to_string(&self) -> String {
        String::from(match self {
            ItemSub::Zujo => "Hat (Zujo)",
            ItemSub::Kami => "Hair (Kami)",
            ItemSub::Hitai => "Forehead (Hitai)",
            ItemSub::Me => "Eyes (Me)",
            ItemSub::Megane => "Glasses (Megane)",
            ItemSub::Mimi => "Ears (Mimi)",
            ItemSub::Kuchi => "Mouth (Kuchi)",
            ItemSub::Maki => "Neck (Maki)",
            ItemSub::Kubi => "Collar (Kubi)",
            ItemSub::Inner => "Body (Inner)",
            ItemSub::Outer => "Outfit (Outer)",
            ItemSub::Kata => "Shoulders (Kata)",
            ItemSub::UUde => "Right Arm (Ude)",
            ItemSub::LUde => "Left Arm (Ude)",
            ItemSub::Te => "Hands (Te)",
            ItemSub::JohaMae => "Chest (Joha Mae)",
            ItemSub::JohaUshiro => "Back (Joha Ushiro)",
            ItemSub::Belt => "Belt",
            ItemSub::Kosi => "Crotch (Kosi)",
            ItemSub::Pants => "Pants",
            ItemSub::Asi => "Legs (Asi)",
            ItemSub::Sune => "Feet (Sune)",
            ItemSub::Kutsu => "Shoes (Kutsu)",
            ItemSub::Hada => "Skin (Hada)",
            ItemSub::Head => "Head",
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Costume {
    pub id: i32,
    pub items: Vec<CostumeItem>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CostumeItem {
    pub id: i32,
    pub objset: Vec<String>,
    pub sub: ItemSub,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Module {
    pub cos: Costume,
    pub chara: Chara,
    pub name: Option<String>,
    pub name_jp: Option<String>,
    pub name_en: Option<String>,
    pub name_cn: Option<String>,
    pub name_fr: Option<String>,
    pub name_ge: Option<String>,
    pub name_it: Option<String>,
    pub name_kr: Option<String>,
    pub name_sp: Option<String>,
    pub name_tw: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CustomizeItem {
    pub bind_module: Option<i32>,
    pub chara: Chara,
    pub part: ItemPart,
    pub obj_id: i32,
    pub name: Option<String>,
    pub name_jp: Option<String>,
    pub name_en: Option<String>,
    pub name_cn: Option<String>,
    pub name_fr: Option<String>,
    pub name_ge: Option<String>,
    pub name_it: Option<String>,
    pub name_kr: Option<String>,
    pub name_sp: Option<String>,
    pub name_tw: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModuleDb {
    pub modules: BTreeMap<i32, Module>,
    pub cstm_items: BTreeMap<i32, CustomizeItem>,
}

impl ModuleDb {
    pub async fn from_files<P: AsRef<std::path::Path>>(
        gm_module_tbl: Option<P>,
        gm_customize_item_tbl: Option<P>,
        chritm_prop: Option<P>,
        mod_str_array: Option<P>,
    ) -> Option<Self> {
        let mut module_db = Self {
            modules: BTreeMap::new(),
            cstm_items: BTreeMap::new(),
        };

        if let Some(gm_module_tbl) = gm_module_tbl {
            if let Some(gm_module_tbl) = parse::Module::parse(gm_module_tbl).await {
                for module in gm_module_tbl.data {
                    let cos = module.cos.trim_start_matches("COS_").parse::<i32>().ok()? - 1;
                    module_db.modules.insert(
                        module.id,
                        Module {
                            cos: Costume {
                                id: cos,
                                items: vec![],
                            },
                            chara: module.chara,
                            name: None,
                            name_jp: Some(module.name.clone()),
                            name_en: None,
                            name_cn: None,
                            name_fr: None,
                            name_ge: None,
                            name_it: None,
                            name_kr: None,
                            name_sp: None,
                            name_tw: None,
                        },
                    );
                }
            }
        }

        if let Some(gm_customize_item_tbl) = gm_customize_item_tbl {
            if let Some(gm_customize_item_tbl) = parse::CstmItem::parse(gm_customize_item_tbl).await
            {
                for cstm_item in gm_customize_item_tbl.data {
                    module_db.cstm_items.insert(
                        cstm_item.id,
                        CustomizeItem {
                            bind_module: cstm_item.bind_module,
                            chara: cstm_item.chara,
                            part: cstm_item.parts,
                            obj_id: cstm_item.obj_id,
                            name: None,
                            name_jp: Some(cstm_item.name.clone()),
                            name_en: None,
                            name_cn: None,
                            name_fr: None,
                            name_ge: None,
                            name_it: None,
                            name_kr: None,
                            name_sp: None,
                            name_tw: None,
                        },
                    );
                }
            }
        }

        if let Some(chritm_prop) = &chritm_prop {
            // Suboptimal, parsing twice here
            let modules = parse::Costume::parse(chritm_prop)
                .await
                .unwrap_or(BTreeMap::new());
            let items = parse::CostumeItem::parse(chritm_prop)
                .await
                .unwrap_or(BTreeMap::new());

            for (_, module) in &mut module_db.modules {
                let Some(costumes) = modules.get(&module.chara) else {
                    println!("Couldnt get costumes for chara");
                    continue;
                };
                let Some(items) = items.get(&module.chara) else {
                    println!("Couldnt get items for chara");
                    continue;
                };
                let Some(cos) = costumes
                    .data
                    .iter()
                    .filter(|cos| cos.id == module.cos.id)
                    .next()
                else {
                    println!("Couldnt get costume {}", module.cos.id);
                    continue;
                };
                for item in &cos.item {
                    let Some(item) = items.data.iter().filter(|itm| itm.no == *item).next() else {
                        println!("Couldnt get item {item} for costume {}", module.cos.id);
                        // Put in a temporary item that we replace later
                        module.cos.items.push(CostumeItem {
                            id: *item,
                            objset: Vec::new(),
                            sub: ItemSub::Te,
                        });
                        continue;
                    };
                    let Ok(item) = item.clone().try_into() else {
                        continue;
                    };
                    module.cos.items.push(item);
                }
            }
        }

        if let Some(mod_str_array) = mod_str_array {
            if let Some(mod_str_array) = parse::ModStringArray::parse(mod_str_array).await {
                for (id, module) in &mut module_db.modules {
                    if let Some(data) = &mod_str_array.data {
                        if let Some(modules) = &data.module {
                            module.name = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.en {
                        if let Some(modules) = &data.module {
                            module.name_en = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.cn {
                        if let Some(modules) = &data.module {
                            module.name_cn = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.fr {
                        if let Some(modules) = &data.module {
                            module.name_fr = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.ge {
                        if let Some(modules) = &data.module {
                            module.name_ge = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.it {
                        if let Some(modules) = &data.module {
                            module.name_it = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.kr {
                        if let Some(modules) = &data.module {
                            module.name_kr = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.sp {
                        if let Some(modules) = &data.module {
                            module.name_sp = modules.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.tw {
                        if let Some(modules) = &data.module {
                            module.name_tw = modules.get(id).cloned();
                        }
                    }
                }
                for (id, cstm_item) in &mut module_db.cstm_items {
                    if let Some(data) = &mod_str_array.data {
                        if let Some(customize) = &data.customize {
                            cstm_item.name = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.en {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_en = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.cn {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_cn = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.fr {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_fr = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.ge {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_ge = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.it {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_it = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.kr {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_kr = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.sp {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_sp = customize.get(id).cloned();
                        }
                    }
                    if let Some(data) = &mod_str_array.tw {
                        if let Some(customize) = &data.customize {
                            cstm_item.name_tw = customize.get(id).cloned();
                        }
                    }
                }
            }
        }

        if module_db.modules.len() == 0 && module_db.cstm_items.len() == 0 {
            None
        } else {
            Some(module_db)
        }
    }

    pub async fn from_folder<P: AsRef<std::path::Path>>(path: P) -> Option<Self> {
        let path = path.as_ref();
        if !path.is_dir() {
            return None;
        }

        let module_tbl = path.join("mod_gm_module_tbl.farc");
        let module_tbl = if module_tbl.exists() {
            Some(module_tbl)
        } else {
            None
        };

        let customize_tbl = path.join("mod_gm_customize_item_tbl.farc");
        let customize_tbl = if customize_tbl.exists() {
            Some(customize_tbl)
        } else {
            None
        };

        let chritm_prop = path.join("mod_chritm_prop.farc");
        let chritm_prop = if chritm_prop.exists() {
            Some(chritm_prop)
        } else {
            None
        };

        let mod_str_array = path.join("lang2").join("mod_str_array.toml");
        let mod_str_array = if mod_str_array.exists() {
            Some(mod_str_array)
        } else {
            None
        };

        Self::from_files(module_tbl, customize_tbl, chritm_prop, mod_str_array).await
    }
}
