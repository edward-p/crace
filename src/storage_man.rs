#![allow(non_snake_case)]
use std::rc::Rc;

use crate::data::Data;
use dioxus::prelude::*;
use gloo_file::{File, ObjectUrl};
use gloo_utils::{document, window};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{FileList, FileReader, HtmlInputElement};

#[inline_props]
pub fn StorageMan(cx: Scope) -> Element {
    let file_reader = Rc::new(FileReader::new().unwrap());
    let file_reader1 = file_reader.clone();
    let c: Closure<dyn FnMut()> = Closure::new(move || {
        let v = file_reader1.result().unwrap();

        if let Ok(data) = serde_json::from_str::<Data>(v.as_string().unwrap().as_str()) {
            data.save();
            let _ = web_sys::window().unwrap().alert_with_message("导入成功!");
        } else {
            let _ = web_sys::window()
                .unwrap()
                .alert_with_message("解析文件失败!");
        }
    });
    file_reader.set_onload(Some(c.as_ref().unchecked_ref()));
    c.forget();

    render! {
        h4 { "学习记录" }
        div{
            style:"display:none",
            input{
                id: "import",
                r#type:"file",
                style:"display:none",
                onchange: move |_| {
                    let e:JsValue=document().get_element_by_id("import").unwrap().into();
                    let input:HtmlInputElement=e.into();
                    let files:FileList=input.files().unwrap();
                    if files.length() > 0 {
                        let file=files.get(0).unwrap();
                        file_reader.read_as_text(&file).unwrap();
                    }
                }
            },
        },
        ul{
            class: "storage-man-buttons",
            li{
                a{
                    href: "#",
                    onclick: move |_| {
                        let data = Data::get_from_storage();
                        let value = serde_json::to_string(&data).unwrap();
                        let blob = File::new_with_options("crace_data.json",value.as_str(), Some("octet/stream"),None);
                        let obj_url:ObjectUrl= blob.into();
                        let _ = window().location().assign(&obj_url);
                    },
                    "导出"
                }
            },
            li{
                a{
                    href: "#",
                    onclick: move |_| {
                        let e:JsValue=document().get_element_by_id("import").unwrap().into();
                        let input:HtmlInputElement=e.into();
                        input.click();
                    },
                    "导入"
                }
            },
            li{
                a{
                    href: "#",
                    onclick: move |_| {
                        if web_sys::window().unwrap().confirm_with_message("确定要清除?").unwrap(){
                                Data::clear();
                                let _=web_sys::window().unwrap().alert_with_message("已清除!");
                        }
                    },
                    "清除"
                }
            }
        }
    }
}
