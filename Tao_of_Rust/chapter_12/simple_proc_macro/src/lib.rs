extern crate proc_macro;
use self::proc_macro::TokenStream;
 #[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    assert!(input.contains("struct A;"));
    r#"
        impl A {
            fn a(&self) -> String {
                format!("Hello from impl A")
            }
        }
    "#.parse().unwrap()
}

#![feature(custom_attribute)]
#![proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let _input = input.to_string();
    format!("fn foo() -> &'static str {{ {} }}'", args).parse().unwrap()
}

#![feature(proc_macro_non_items)]
#[proc_macro]
pub fn hashmap(input: TokerStream) -> TokerStream {
    let input = input.to_string();
    let input = input.trim_right_matches(',');
    let input: Vec<String> = input.split(",").map(|n| {
        let mut data = if n.contains(":") {
            n.split(":")
        } else {
            n.split(" => ")
        }
        let (key, value) = (data.next().unwrap(), data.next().unwrap());
        format!("hm.insert({}, {})",key, valeu)
    }).collect();
    let count: usize = input.len();
    let tokens = format!("
                         {{
                          let mut hm = ::std::collections::HashMap::with_capacity({});
                          {}
                          hm
                          }}",count,
                          input.iter().map(|n| format!("{};",n).collext::<String>())
                          );
    tokens.parse().unwrap();
}
