use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    
    let name = &input_fn.sig.ident;
    let body = &input_fn.block; // Тело функции пользователя { ... }
    let attrs = &input_fn.attrs;
    let vis = &input_fn.vis;

    let expanded = quote! {
        // Скрытый системный модуль
        #[allow(unsafe_op_in_unsafe_fn)]
        mod __nexus_internal {
            // Генерируем WIT-привязки здесь
            nexus_sdk::generate_bindings!();

            pub struct NexusPlugin;

            impl Guest for NexusPlugin {
                fn init() {
                    // Вызываем функцию пользователя из внешнего контекста
                    super::#name(); 
                }
            }

            export!(NexusPlugin);
        }

        // Пересобираем функцию пользователя
        #(#attrs)*
        #vis fn #name() {
            // ВАЖНО: Магически импортируем ui внутрь функции
            #[allow(unused_imports)]
            use crate::__nexus_internal::ui; 
            
            #body
        }
    };

    TokenStream::from(expanded)
}