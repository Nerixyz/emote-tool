#[macro_export]
macro_rules! getter_setter {
    ($name:ident,$type:ty) => {
        pub fn $name(&self) -> $type {
            self.conf.$name
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: $type) -> &mut Self {
                self.conf.$name = $name;
                self
            }
        });
    };
    ($c_name:ident as $name:ident,$type:ty) => {
        pub fn $name(&self) -> $type {
            self.conf.$c_name
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: $type) -> &mut Self {
                self.conf.$c_name = $name;
                self
            }
        });
    };
    ($name:ident!) => {
        pub fn $name(&self) -> bool {
            self.conf.$name != 0
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: bool) -> &mut Self {
                self.conf.$name = if $name {1} else {0};
                self
            }
        });
    };
    ($c_name:ident as $name:ident!) => {
        pub fn $name(&self) -> bool {
            self.conf.$c_name != 0
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: bool) -> &mut Self {
                self.conf.$c_name = if $name {1} else {0};
                self
            }
        });
    };
    ($name:ident,into $type:ty) => {
        pub fn $name(&self) -> $type {
            self.conf.$name.into()
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: $type) -> &mut Self {
                self.conf.$name = $name.into();
                self
            }
        });
    };
    ($name:ident,$type:ty: clamp $min:literal to $max:literal) => {
        pub fn $name(&self) -> $type {
            self.conf.$name
        }
        concat_idents::concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, $name: $type) -> &mut Self {
                self.conf.$name = $name.clamp($min, $max);
                self
            }
        });
    };
}

#[macro_export]
macro_rules! getter_debug {
    ($self:ident, $formatter:ident, $struct_name:literal; $($name:ident,)*) => {
        $formatter.debug_struct($struct_name)
            $(.field(stringify!($name), &$self.$name()))*
            .finish()
    };
}
