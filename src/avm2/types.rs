use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub struct AbcFile<'a> {
    pub major_version: u16,
    pub minor_version: u16,
    pub constant_pool: ConstantPool<'a>,
    pub methods: Vec<Method<'a>>,
    pub metadata: Vec<Metadata<'a>>,
    pub instances: Vec<Instance<'a>>,
    pub classes: Vec<Class<'a>>,
    pub scripts: Vec<Script<'a>>,
    pub method_bodies: Vec<MethodBody<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantPool<'a> {
    pub ints: Vec<i32>,
    pub uints: Vec<u32>,
    pub doubles: Vec<f64>,
    pub strings: Vec<&'a str>,
    pub namespaces: Vec<Namespace<'a>>,
    pub namespace_sets: Vec<NamespaceSet<'a>>,
    pub multinames: Vec<Multiname<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Index<T>(pub u32, pub PhantomData<T>);

impl<T> Index<T> {
    pub fn new(i: u32) -> Index<T> {
        Index(i, PhantomData)
    }

    pub fn as_u30(&self) -> u32 {
        self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Namespace<'a> {
    Namespace(Index<&'a str>),
    Package(Index<&'a str>),
    PackageInternal(Index<&'a str>),
    Protected(Index<&'a str>),
    Explicit(Index<&'a str>),
    StaticProtected(Index<&'a str>),
    Private(Index<&'a str>),
}

pub type NamespaceSet<'a> = Vec<Index<Namespace<'a>>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Multiname<'a> {
    QName {
        namespace: Index<Namespace<'a>>,
        name: Index<&'a str>,
    },
    QNameA {
        namespace: Index<Namespace<'a>>,
        name: Index<&'a str>,
    },
    RTQName {
        name: Index<&'a str>,
    },
    RTQNameA {
        name: Index<&'a str>,
    },
    RTQNameL,
    RTQNameLA,
    Multiname {
        namespace_set: Index<NamespaceSet<'a>>,
        name: Index<&'a str>,
    },
    MultinameA {
        namespace_set: Index<NamespaceSet<'a>>,
        name: Index<&'a str>,
    },
    MultinameL {
        namespace_set: Index<NamespaceSet<'a>>,
    },
    MultinameLA {
        namespace_set: Index<NamespaceSet<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Method<'a> {
    pub name: Index<&'a str>,
    pub params: Vec<MethodParam<'a>>,
    pub return_type: Index<Multiname<'a>>,
    pub needs_arguments_object: bool,
    pub needs_activation: bool,
    pub needs_rest: bool,
    pub needs_dxns: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodParam<'a> {
    pub name: Option<Index<&'a str>>,
    pub kind: Index<Multiname<'a>>,
    pub default_value: Option<DefaultValue<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodBody<'a> {
    pub method: Index<Method<'a>>,
    pub max_stack: u32,
    pub num_locals: u32,
    pub init_scope_depth: u32,
    pub max_scope_depth: u32,
    pub code: &'a [u8],
    pub exceptions: Vec<Exception<'a>>,
    pub traits: Vec<Trait<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Exception<'a> {
    pub from_offset: u32,
    pub to_offset: u32,
    pub target_offset: u32,
    pub variable_name: Index<&'a str>,
    pub type_name: Index<Multiname<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Opcode;

#[derive(Clone, Debug, PartialEq)]
pub enum DefaultValue<'a> {
    Int(Index<i32>),
    Uint(Index<u32>),
    Double(Index<f64>),
    String(Index<&'a str>),
    True,
    False,
    Null,
    Undefined,
    Namespace(Index<Namespace<'a>>),
    Package(Index<Namespace<'a>>),
    PackageInternal(Index<Namespace<'a>>),
    Protected(Index<Namespace<'a>>),
    Explicit(Index<Namespace<'a>>),
    StaticProtected(Index<Namespace<'a>>),
    Private(Index<Namespace<'a>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Metadata<'a> {
    pub name: Index<&'a str>,
    pub items: Vec<MetadataItem<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MetadataItem<'a> {
    pub key: Index<&'a str>,
    pub value: Index<&'a str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instance<'a> {
    pub name: Index<Multiname<'a>>,
    pub super_name: Index<Multiname<'a>>,
    pub is_sealed: bool,
    pub is_final: bool,
    pub is_interface: bool,
    pub protected_namespace: Option<Index<Namespace<'a>>>,
    pub interfaces: Vec<Index<Multiname<'a>>>,
    pub init_method: Index<Method<'a>>,
    pub traits: Vec<Trait<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Trait<'a> {
    pub name: Index<Multiname<'a>>,
    pub kind: TraitKind<'a>,
    pub metadata: Vec<Index<Metadata<'a>>>,
    pub is_final: bool,
    pub is_override: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TraitKind<'a> {
    Slot {
        slot_id: u32,
        type_name: Index<Multiname<'a>>,
        value: Option<DefaultValue<'a>>,
    },
    Method {
        disp_id: u32,
        method: Index<Method<'a>>,
    },
    Getter {
        disp_id: u32,
        method: Index<Method<'a>>,
    },
    Setter {
        disp_id: u32,
        method: Index<Method<'a>>,
    },
    Class {
        slot_id: u32,
        class: Index<Class<'a>>,
    },
    Function {
        slot_id: u32,
        function: Index<Method<'a>>,
    },
    Const {
        slot_id: u32,
        type_name: Index<Multiname<'a>>,
        value: Option<DefaultValue<'a>>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Class<'a> {
    pub init_method: Index<Method<'a>>,
    pub traits: Vec<Trait<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Script<'a> {
    pub init_method: Index<Method<'a>>,
    pub traits: Vec<Trait<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op<'a> {
    Add,
    AddI,
    AsType {
        type_name: Index<Multiname<'a>>,
    },
    AsTypeLate,
    BitAnd,
    BitNot,
    BitOr,
    BitXor,
    Call {
        num_args: u32,
    },
    CallMethod {
        index: Index<Method<'a>>,
        num_args: u32,
    },
    CallProperty {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    CallPropLex {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    CallPropVoid {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    CallStatic {
        index: Index<Method<'a>>,
        num_args: u32,
    },
    CallSuper {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    CallSuperVoid {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    CheckFilter,
    Coerce {
        index: Index<Multiname<'a>>,
    },
    CoerceA,
    CoerceS,
    Construct {
        num_args: u32,
    },
    ConstructProp {
        index: Index<Multiname<'a>>,
        num_args: u32,
    },
    ConstructSuper {
        num_args: u32,
    },
    ConvertB,
    ConvertD,
    ConvertI,
    ConvertO,
    ConvertS,
    ConvertU,
    Debug {
        is_local_register: bool,
        register_name: Index<&'a str>,
        register: u8,
    },
    DebugFile {
        file_name: Index<&'a str>,
    },
    DebugLine {
        line_num: u32,
    },
    DecLocal {
        index: u32,
    },
    DecLocalI {
        index: u32,
    },
    Decrement,
    DecrementI,
    DeleteProperty {
        index: Index<Multiname<'a>>,
    },
    Divide,
    Dup,
    Dxns {
        index: Index<&'a str>,
    },
    DxnsLate,
    Equals,
    EscXAttr,
    EscXElem,
    FindProperty {
        index: Index<Multiname<'a>>,
    },
    FindPropStrict {
        index: Index<Multiname<'a>>,
    },
    GetDescendants {
        index: Index<Multiname<'a>>,
    },
    GetGlobalScope,
    GetGlobalSlot {
        index: u32,
    },
    GetLex {
        index: Index<Multiname<'a>>,
    },
    GetLocal {
        index: u32,
    },
    GetProperty {
        index: Index<Multiname<'a>>,
    },
    GetScopeObject {
        index: u8,
    },
    GetSlot {
        index: u32,
    },
    GetSuper {
        index: Index<Multiname<'a>>,
    },
    GreaterEquals,
    GreaterThan,
    HasNext,
    HasNext2 {
        object_register: u32,
        index_register: u32,
    },
    IfEq {
        offset: i32,
    },
    IfFalse {
        offset: i32,
    },
    IfGe {
        offset: i32,
    },
    IfGt {
        offset: i32,
    },
    IfLe {
        offset: i32,
    },
    IfLt {
        offset: i32,
    },
    IfNge {
        offset: i32,
    },
    IfNgt {
        offset: i32,
    },
    IfNle {
        offset: i32,
    },
    IfNlt {
        offset: i32,
    },
    IfNe {
        offset: i32,
    },
    IfStrictEq {
        offset: i32,
    },
    IfStrictNe {
        offset: i32,
    },
    IfTrue {
        offset: i32,
    },
    In,
    IncLocal {
        index: u32,
    },
    IncLocalI {
        index: u32,
    },
    Increment,
    IncrementI,
    InitProperty {
        index: Index<Multiname<'a>>,
    },
    InstanceOf,
    IsType {
        index: Index<Multiname<'a>>,
    },
    IsTypeLate,
    Jump {
        offset: i32,
    },
    Kill {
        index: u32,
    },
    Label,
    LessEquals,
    LessThan,
    LookupSwitch {
        default_offset: i32,
        case_offsets: Vec<i32>,
    },
    LShift,
    Modulo,
    Multiply,
    MultiplyI,
    Negate,
    NegateI,
    NewActivation,
    NewArray {
        num_args: u32,
    },
    NewCatch {
        index: Index<Exception<'a>>,
    },
    NewClass {
        index: Index<Class<'a>>,
    },
    NewFunction {
        index: Index<Method<'a>>,
    },
    NewObject {
        num_args: u32,
    },
    NextName,
    NextValue,
    Nop,
    Not,
    Pop,
    PopScope,
    PushByte {
        value: u8,
    },
    PushDouble {
        value: Index<f64>,
    },
    PushFalse,
    PushInt {
        value: Index<i32>,
    },
    PushNamespace {
        value: Index<Namespace<'a>>,
    },
    PushNaN,
    PushNull,
    PushScope,
    PushShort {
        value: u32,
    }, // TODO: Is this really a u30?
    PushString {
        value: Index<&'a str>,
    },
    PushTrue,
    PushUint {
        value: Index<u32>,
    },
    PushUndefined,
    PushWith,
    ReturnValue,
    ReturnVoid,
    RShift,
    SetLocal {
        index: u32,
    },
    SetGlobalSlot {
        index: u32,
    },
    SetProperty {
        index: Index<Multiname<'a>>,
    },
    SetSlot {
        index: u32,
    },
    SetSuper {
        index: Index<Multiname<'a>>,
    },
    StrictEquals,
    Subtract,
    SubtractI,
    Swap,
    Throw,
    TypeOf,
    URShift,
}
