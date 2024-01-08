use 
::Value;
//use std::collections::HashMap;
use std::io::{self};

fn main() {
    struct JsonEditor {
        json_data: Value,
    }

    impl JsonEditor {
        fn new(json_string: &str) -> Result<Self, serde_json::Error> {
            let json_data = serde_json::from_str(json_string)?;
            Ok(JsonEditor { json_data })
        }

        fn find(&self, key: &str) -> Option<&Value> {
            self.json_data.get(key)
        }

        fn delete(&mut self, key: &str) {
            if let Some(obj) = self.json_data.as_object_mut() {
                obj.remove(key);
            }
        }

        fn update(&mut self, key: &str, value: Value) {
            if let Some(obj) = self.json_data.as_object_mut() {
                obj.insert(key.to_string(), value);
            }
        }

        fn get_final_json(&self) -> String {
            self.json_data.to_string()
        }

        fn print_menu() {
            println!("1. 查找参数");
            println!("2. 删除参数");
            println!("3. 更新参数");
            println!("4. 打印最终的 JSON 数据");
            println!("5. 退出");
        }
    }

    let json_str = r#"[
        {
            "name": "网址",
            "idType": "ip",
            "idIndex": "2000",
            "metadata": {
                "type": "string",
                "minLength": 1,
                "maxLength": 10
            },
            "required": false
        },
        {
            "name": "端口号",
            "idType": "port",
            "idIndex": "2001",
            "metadata": {
                "type": "string",
                "minLength": 1,
                "maxLength": 10
            },
            "required": false
        },
        {
            "name": "域名",
            "idType": "DNS",
            "idIndex": "2003",
            "metadata": {
                "type": "string",
                "minLength": 1,
                "maxLength": 100
            },
            "required": false
        },
        {
            "name": "备注",
            "idType": "memo",
            "idIndex": "2004",
            "metadata": {
                "type": "string",
                "minLength": 1,
                "maxLength": 200
            },
            "required": false
        }
    ]"#;

    let mut editor = match JsonEditor::new(json_str) {
        Ok(editor) => editor,
        Err(err) => {
            eprintln!("Error parsing JSON: {}", err);
            return;
        }
    };

    loop {
        JsonEditor::print_menu();
        println!("请选择操作:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");

        match input.trim().parse() {
            Ok(choice) => {
                match choice {
                    1 => {
                        println!("请输入要查找的参数名称:");
                        let mut key_input = String::new();
                        io::stdin().read_line(&mut key_input).expect("无法读取输入");
                        let key = key_input.trim();
                        if let Some(value) = editor.find(key) {
                            println!("找到参数 {}: {}", key, value);
                        } else {
                            println!("参数 {} 未找到", key);
                        }
                    }
                    2 => {
                        println!("请输入要删除的参数名称:");
                        let mut key_input = String::new();
                        io::stdin().read_line(&mut key_input).expect("无法读取输入");
                        let key = key_input.trim();
                        editor.delete(key);
                        println!("参数 {} 已删除", key);
                    }
                    3 => {
                        println!("请输入要更新的参数名称:");
                        let mut key_input = String::new();
                        io::stdin().read_line(&mut key_input).expect("无法读取输入");
                        let key = key_input.trim();

                        println!("请输入新的值:");
                        let mut value_input = String::new();
                        io::stdin().read_line(&mut value_input).expect("无法读取输入");
                        let value = value_input.trim();

                        editor.update(key, serde_json::from_str(value).expect("无法解析值"));
                        println!("参数 {} 已更新为 {}", key, value);
                    }
                    4 => {
                        let final_json = editor.get_final_json();
                        println!("最终 JSON 数据:\n{}", final_json);
                    }
                    5 => {
                        println!("退出程序");
                        return;
                    }
                    _ => println!("无效选项，请重新选择"),
                }
            }
            Err(_) => {
                println!("输入无效，请重新选择");
            }
        }
    }
}
