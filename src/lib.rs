use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

mod parse;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Chara {
    #[serde(rename = "MIK")]
    Miku,
    #[serde(rename = "RIN")]
    Rin,
    #[serde(rename = "LEN")]
    Len,
    #[serde(rename = "LUK")]
    Luka,
    #[serde(rename = "NER")]
    Neru,
    #[serde(rename = "HAK")]
    Haku,
    #[serde(rename = "KAI")]
    Kaito,
    #[serde(rename = "MEI")]
    Meiko,
    #[serde(rename = "SAK")]
    Sakine,
    #[serde(rename = "TET")]
    Teto,
    #[serde(rename = "EXT")]
    Extra,
    #[serde(rename = "ALL")]
    All,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ItemPart {
    Kami,
    Face,
    Neck,
    Zujo,
    Back,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Costume {
    pub id: i32,
    pub items: Vec<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomizeItem {
    pub bind_module: Option<i32>,
    pub chara: Chara,
    pub part: ItemPart,
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
            let gm_module_tbl = parse::Module::parse(gm_module_tbl).await?;
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

        if let Some(gm_customize_item_tbl) = gm_customize_item_tbl {
            let gm_customize_item_tbl = parse::CstmItem::parse(gm_customize_item_tbl).await?;
            for cstm_item in gm_customize_item_tbl.data {
                module_db.cstm_items.insert(
                    cstm_item.id,
                    CustomizeItem {
                        bind_module: cstm_item.bind_module,
                        chara: cstm_item.chara,
                        part: cstm_item.parts,
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

        if let Some(chritm_prop) = chritm_prop {
            let chritm_prop = parse::Costume::parse(chritm_prop).await?;
            for (_, module) in &mut module_db.modules {
                if let Some(prop) = chritm_prop.get(&module.chara) {
                    if let Some(cos) = prop
                        .data
                        .iter()
                        .filter(|cos| cos.id == module.cos.id)
                        .next()
                    {
                        module.cos.items = cos.item.clone();
                    }
                }
            }
        }

        if let Some(mod_str_array) = mod_str_array {
            let mod_str_array = parse::ModStringArray::parse(mod_str_array).await?;
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
