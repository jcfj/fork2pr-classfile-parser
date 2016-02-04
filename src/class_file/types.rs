use constant_info::ConstantInfo;
use field_info::FieldInfo;
use method_info::MethodInfo;
use attribute_info::AttributeInfo;

pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool_size: u16,
    pub const_pool: Vec<ConstantInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

// pub enum ClassAccessFlags {
//     Public,     // 	0x0001 	Declared public; may be accessed from outside its package.
//     Final,      // 	0x0010 	Declared final; no subclasses allowed.
//     Super,      // 	0x0020 	Treat superclass methods specially when invoked by the invokespecial instruction.
//     Interface,  // 	0x0200 	Is an interface, not a class.
//     Abstract,   // 	0x0400 	Declared abstract; must not be instantiated.
//     Synthetic,  // 	0x1000 	Declared synthetic; not present in the source code.
//     Annotation, // 	0x2000 	Declared as an annotation type.
//     Enum,       // 	0x4000 	Declared as an enum type.
// }