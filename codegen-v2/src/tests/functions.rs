use crate::grammar::{
    GFuncName, GFunctionDecl, GMarker, GMarkers, GParamItem, GParamName, GPrimitive, GReturnValue,
    GStruct, GType, GTypeCategory, ParseTree,
};
use crate::reader::Reader;
use crate::{must_err, must_ok};

#[test]
fn test_func_params_with_marker() {
    must_ok!(
        GParamItem,
        "int _Nonnull my_var\n",
        GParamItem {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Int)),
            name: GParamName::from("my_var"),
            markers: GMarkers(vec![GMarker::NonNull]),
        }
    );

    must_ok!(
        GParamItem,
        "bool\n_Nonnull  some_bool\n",
        GParamItem {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Bool)),
            name: GParamName::from("some_bool"),
            markers: GMarkers(vec![GMarker::NonNull]),
        }
    );
}

#[test]
fn test_func_params_without_marker() {
    must_ok!(
        GParamItem,
        "int my_var",
        GParamItem {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Int)),
            name: GParamName::from("my_var"),
            markers: GMarkers(vec![]),
        }
    );

    must_ok!(
        GParamItem,
        "bool \nsome_bool",
        GParamItem {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Bool)),
            name: GParamName::from("some_bool"),
            markers: GMarkers(vec![]),
        }
    );
}

#[test]
fn test_function_delceration() {
    let expected = GFunctionDecl {
        name: GFuncName::from("some_function"),
        params: vec![
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Int)),
                name: GParamName::from("some_int"),
                markers: GMarkers(vec![]),
            },
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Bool)),
                name: GParamName::from("some_bool"),
                markers: GMarkers(vec![]),
            },
        ],
        return_value: GReturnValue {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Void)),
            markers: GMarkers(vec![]),
        },
        markers: GMarkers(vec![]),
    };

    must_ok!(
        GFunctionDecl,
        "void some_function(int some_int, bool some_bool);",
        expected
    );

    must_ok!(
        GFunctionDecl,
        "void some_function(int some_int ,bool some_bool);",
        expected
    );

    must_ok!(
        GFunctionDecl,
        "void some_function(int some_int,bool some_bool);",
        expected
    );

    must_ok!(
        GFunctionDecl,
        "void some_function(int some_int , bool some_bool);",
        expected
    );

    must_err!(
        GFunctionDecl,
        "void some_function(int some_int bool some_bool);"
    );
}

#[test]
fn test_function_delceration_with_markers() {
    let expected = GFunctionDecl {
        name: GFuncName::from("some_function"),
        params: vec![
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Int)),
                name: GParamName::from("some_int"),
                markers: GMarkers(vec![]),
            },
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Bool)),
                name: GParamName::from("some_bool"),
                markers: GMarkers(vec![]),
            },
        ],
        return_value: GReturnValue {
            ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Void)),
            markers: GMarkers(vec![]),
        },
        markers: GMarkers(vec![GMarker::TwExportStruct, GMarker::TWVisibilityDefault]),
    };

    must_ok!(
        GFunctionDecl,
        "TW_EXPORT_STRUCT void some_function(int some_int, bool some_bool) TW_VISIBILITY_DEFAULT;",
        expected
    );
}

#[test]
fn test_function_delceration_struct_return_value() {
    let expected = GFunctionDecl {
        name: GFuncName::from("some_function"),
        params: vec![
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Int)),
                name: GParamName::from("some_int"),
                markers: GMarkers(vec![]),
            },
            GParamItem {
                ty: GType::Mutable(GTypeCategory::Scalar(GPrimitive::Bool)),
                name: GParamName::from("some_bool"),
                markers: GMarkers(vec![]),
            },
        ],
        return_value: GReturnValue {
            ty: GType::Mutable(GTypeCategory::Pointer(Box::new(GTypeCategory::Struct(
                GStruct::from("SomeStruct"),
            )))),
            markers: GMarkers(vec![GMarker::Nullable]),
        },
        markers: GMarkers(vec![
            GMarker::TwExportStaticMethod,
            GMarker::TWVisibilityDefault,
        ]),
    };

    must_ok!(
        GFunctionDecl,
        "TW_EXPORT_STATIC_METHOD struct SomeStruct* _Nullable some_function(int some_int, bool some_bool) TW_VISIBILITY_DEFAULT;",
        expected
    );
}
