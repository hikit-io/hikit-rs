#[macro_export]
macro_rules! struct_fields {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($(#[$vmeta:meta])* $vname:ident : $tname:ty,)*
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $($(#[$vmeta])* $vname:$tname,)+
        }
        paste::paste!{
            #[derive(Debug)]
            $vis struct [<$name Fields>] {
                $(pub $vname:&'static str,)+
            }
        }
        // impl $name {
        //     $vis fn fields() -> [<$name Fields>]{
        //         [<$name Fields>]{
        //             $(pub $vname:$vname,)+
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
mod tests {
    struct_fields! {
        #[derive(Clone)]
        pub struct TestStruct{
             _id:String,
        }
    }
    #[test]
    fn it_works() {
        // assert_eq!(TestStruct::fields()._id, "_id");
    }
}
