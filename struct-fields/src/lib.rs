#[macro_export]
macro_rules! struct_fields {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($(#[$vmeta:meta])* $fieldVis:vis $vname:ident : $tname:ty,)*
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($(#[$vmeta])* $fieldVis $vname:$tname,)+
        }
        affix::paste!{
            #[derive(Debug)]
            $vis struct [<$name Fields>] {
                $(pub $vname:&'static str,)+
            }

            impl $name{
               $vis fn fields() -> &'static [<$name Fields>] {
                    &[<$name Fields>] {
                        $($vname:stringify!([<$vname:camel>]),)+
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    struct_fields! {
        #[derive(Clone)]
        pub struct TestStruct{
             _id:String,
            name:String,
        }
    }
    #[test]
    fn it_works() {
        assert_eq!(TestStruct::fields()._id, "id");
        assert_eq!(TestStruct::fields().name, "name");
    }
}
