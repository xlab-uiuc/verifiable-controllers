// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::api_resource::*;
use crate::kubernetes_api_objects::common::*;
use crate::kubernetes_api_objects::dynamic::*;
use crate::kubernetes_api_objects::marshal::*;
use crate::kubernetes_api_objects::object_meta::*;
use crate::kubernetes_api_objects::resource::*;
use crate::pervasive_ext::string_map::StringMap;
use crate::pervasive_ext::string_view::StringView;
use vstd::prelude::*;
use vstd::seq_lib::*;
use vstd::string::*;
use vstd::vec::*;

verus! {


/// This definition is a wrapper of RoleBinding defined at
/// https://github.com/Arnavion/k8s-openapi/blob/v0.17.0/src/v1_26/api/rbac/v1/role_binding.rs.
/// It is supposed to be used in exec controller code.
///
/// More detailed information: https://kubernetes.io/docs/reference/access-authn-authz/rbac/.

#[verifier(external_body)]
pub struct RoleBinding {
    inner: deps_hack::k8s_openapi::api::rbac::v1::RoleBinding,
}

impl RoleBinding {
    pub spec fn view(&self) -> RoleBindingView;

    #[verifier(external_body)]
    pub fn default() -> (role_binding: RoleBinding)
        ensures
            role_binding@ == RoleBindingView::default(),
    {
        RoleBinding {
            inner: deps_hack::k8s_openapi::api::rbac::v1::RoleBinding::default(),
        }
    }

    #[verifier(external_body)]
    pub fn metadata(&self) -> (metadata: ObjectMeta)
        ensures
            metadata@ == self@.metadata,
    {
        todo!()
    }


    #[verifier(external_body)]
    pub fn set_metadata(&mut self, metadata: ObjectMeta)
        ensures
            self@ == old(self)@.set_metadata(metadata@),
    {
        self.inner.metadata = metadata.into_kube();
    }

    #[verifier(external_body)]
    pub fn set_role_ref(&mut self, role_ref: RoleRef)
        ensures
            self@ == old(self)@.set_role_ref(role_ref@),
    {
        self.inner.role_ref = role_ref.into_kube();
    }

    #[verifier(external_body)]
    pub fn set_subjects(&mut self, subjects: Vec<Subject>)
        ensures
            self@ == old(self)@.set_subjects(subjects@.map_values(|s: Subject| s@)),
    {
        self.inner.subjects = std::option::Option::Some(
            subjects.vec.into_iter().map(|s: Subject| s.into_kube()).collect()
        );
    }


    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::rbac::v1::RoleBinding {
        self.inner
    }

    #[verifier(external_body)]
    pub fn api_resource() -> (res: ApiResource)
        ensures
            res@.kind == Kind::CustomResourceKind,
    {
        ApiResource::from_kube(deps_hack::kube::api::ApiResource::erase::<deps_hack::k8s_openapi::api::rbac::v1::RoleBinding>(&()))
    }

    #[verifier(external_body)]
    pub fn to_dynamic_object(self) -> (obj: DynamicObject)
        ensures
            obj@ == self@.to_dynamic_object(),
    {
        DynamicObject::from_kube(
            deps_hack::k8s_openapi::serde_json::from_str(&deps_hack::k8s_openapi::serde_json::to_string(&self.inner).unwrap()).unwrap()
        )
    }

    #[verifier(external_body)]
    pub fn from_dynamic_object(obj: DynamicObject) -> (svc: RoleBinding)
        ensures
            svc@ == RoleBindingView::from_dynamic_object(obj@),
    {
        RoleBinding { inner: obj.into_kube().try_parse::<deps_hack::k8s_openapi::api::rbac::v1::RoleBinding>().unwrap() }
    }
}

#[verifier(external_body)]
pub struct RoleRef {
    inner: deps_hack::k8s_openapi::api::rbac::v1::RoleRef,
}

impl RoleRef {
    pub spec fn view(&self) -> RoleRefView;

    #[verifier(external_body)]
    pub fn default() -> (role_ref: RoleRef)
        ensures
            role_ref@ == RoleRefView::default(),
    {
        RoleRef {
            inner: deps_hack::k8s_openapi::api::rbac::v1::RoleRef::default(),
        }
    }

    #[verifier(external_body)]
    pub fn set_api_group(&mut self, api_group: String)
        ensures
            self@ == old(self)@.set_api_group(api_group@),
    {
        self.inner.api_group = api_group.into_rust_string();
    }

    #[verifier(external_body)]
    pub fn set_kind(&mut self, kind: String)
        ensures
            self@ == old(self)@.set_kind(kind@),
    {
        self.inner.kind = kind.into_rust_string();
    }

    #[verifier(external_body)]
    pub fn set_name(&mut self, name: String)
        ensures
            self@ == old(self)@.set_name(name@),
    {
        self.inner.name = name.into_rust_string();
    }


    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::rbac::v1::RoleRef {
        self.inner
    }
}



#[verifier(external_body)]
pub struct Subject {
    inner: deps_hack::k8s_openapi::api::rbac::v1::Subject,
}

impl Subject {
    pub spec fn view(&self) -> SubjectView;

    #[verifier(external_body)]
    pub fn default() -> (subject: Subject)
        ensures
            subject@ == SubjectView::default(),
    {
        Subject {
            inner: deps_hack::k8s_openapi::api::rbac::v1::Subject::default(),
        }
    }

    #[verifier(external_body)]
    pub fn set_kind(&mut self, kind: String)
        ensures
            self@ == old(self)@.set_kind(kind@),
    {
        self.inner.kind = kind.into_rust_string();
    }

    #[verifier(external_body)]
    pub fn set_name(&mut self, name: String)
        ensures
            self@ == old(self)@.set_name(name@),
    {
        self.inner.name = name.into_rust_string();
    }

    #[verifier(external_body)]
    pub fn set_namespace(&mut self, namespace: String)
        ensures
            self@ == old(self)@.set_namespace(namespace@),
    {
        self.inner.namespace = std::option::Option::Some(namespace.into_rust_string());
    }

    #[verifier(external)]
    pub fn into_kube(self) -> deps_hack::k8s_openapi::api::rbac::v1::Subject {
        self.inner
    }
}




/// RoleBindingView is the ghost type of RoleBinding.
/// It is supposed to be used in spec and proof code.

pub struct RoleBindingView {
    pub metadata: ObjectMetaView,
    pub role_ref: RoleRefView,
    pub subjects: Option<Seq<SubjectView>>,
}

impl RoleBindingView {
    pub open spec fn default() -> RoleBindingView {
        RoleBindingView {
            metadata: ObjectMetaView::default(),
            role_ref: RoleRefView::default(),
            subjects: Option::None,
        }
    }

    pub open spec fn set_metadata(self, metadata: ObjectMetaView) -> RoleBindingView {
        RoleBindingView {
            metadata: metadata,
            ..self
        }
    }

    pub open spec fn set_role_ref(self, role_ref: RoleRefView) -> RoleBindingView {
        RoleBindingView {
            role_ref: role_ref,
            ..self
        }
    }

    pub open spec fn set_subjects(self, subjects: Seq<SubjectView>) -> RoleBindingView {
        RoleBindingView {
            subjects: Option::Some(subjects),
            ..self
        }
    }

    pub open spec fn role_ref_field() -> nat {0}

    pub open spec fn subjects_field() -> nat {1}
}

impl ResourceView for RoleBindingView {
    open spec fn metadata(self) -> ObjectMetaView {
        self.metadata
    }

    open spec fn kind(self) -> Kind {
        Kind::RoleBindingKind
    }

    open spec fn object_ref(self) -> ObjectRef {
        ObjectRef {
            kind: self.kind(),
            name: self.metadata.name.get_Some_0(),
            namespace: self.metadata.namespace.get_Some_0(),
        }
    }

    open spec fn to_dynamic_object(self) -> DynamicObjectView {
        DynamicObjectView {
            kind: self.kind(),
            metadata: self.metadata,
            data: Value::Object(Map::empty()
                .insert(Self::role_ref_field(), self.role_ref.marshal())
                .insert(Self::subjects_field(), if self.subjects.is_None() {Value::Null} else {
                    Value::Array(self.subjects.get_Some_0().map_values(|v:SubjectView| v.marshal()))
                })
            ),
        }
    }

    open spec fn from_dynamic_object(obj: DynamicObjectView) -> RoleBindingView {
        RoleBindingView {
            metadata: obj.metadata,
            role_ref: RoleRefView::unmarshal(obj.data.get_Object_0()[Self::role_ref_field()]),
            subjects: if obj.data.get_Object_0()[Self::subjects_field()].is_Null() {Option::None} else {
                Option::Some(obj.data.get_Object_0()[Self::subjects_field()].get_Array_0().map_values(|v: Value| SubjectView::unmarshal(v)))
            },
        }
    }

    proof fn to_dynamic_preserves_integrity() {
        assert forall |o: Self| o == Self::from_dynamic_object(#[trigger] o.to_dynamic_object()) by {
            if o.subjects.is_Some() {
                SubjectView::marshal_preserves_integrity();
                assert_seqs_equal!(
                    o.subjects.get_Some_0(),
                    Self::from_dynamic_object(o.to_dynamic_object()).subjects.get_Some_0()
                );
            }
        }
    }
}

pub struct RoleRefView {
    pub api_group: StringView,
    pub kind: StringView,
    pub name: StringView,
}

impl RoleRefView {
    pub open spec fn default() -> RoleRefView {
        RoleRefView {
            api_group: new_strlit("")@,
            kind: new_strlit("")@,
            name: new_strlit("")@,
        }
    }

    pub open spec fn set_api_group(self, api_group: StringView) -> RoleRefView {
        RoleRefView {
            api_group: api_group,
            ..self
        }
    }

    pub open spec fn set_kind(self, kind: StringView) -> RoleRefView {
        RoleRefView {
            kind: kind,
            ..self
        }
    }

    pub open spec fn set_name(self, name: StringView) -> RoleRefView {
        RoleRefView {
            name: name,
            ..self
        }
    }

    pub open spec fn api_group_field() -> nat {0}

    pub open spec fn kind_field() -> nat {1}

    pub open spec fn name_field() -> nat {2}
}

impl Marshalable for RoleRefView {
    open spec fn marshal(self) -> Value {
        Value::Object(
            Map::empty()
                .insert(Self::api_group_field(), Value::String(self.api_group))
                .insert(Self::kind_field(), Value::String(self.kind))
                .insert(Self::name_field(), Value::String(self.name))
        )
    }

    open spec fn unmarshal(value: Value) -> Self {
        RoleRefView {
            api_group: value.get_Object_0()[Self::api_group_field()].get_String_0(),
            kind: value.get_Object_0()[Self::kind_field()].get_String_0(),
            name: value.get_Object_0()[Self::name_field()].get_String_0(),
        }
    }

    proof fn marshal_preserves_integrity() {
        assert forall |o: Self| o == Self::unmarshal(#[trigger] o.marshal()) by {
        }
    }

}



pub struct SubjectView {
    pub kind: StringView,
    pub name: StringView,
    pub namespace: Option<StringView>,
}


impl SubjectView {
    pub open spec fn default() -> SubjectView {
        SubjectView {
            kind: new_strlit("")@,
            name: new_strlit("")@,
            namespace: Option::None,
        }
    }

    pub open spec fn set_kind(self, kind: StringView) -> SubjectView {
        SubjectView {
            kind: kind,
            ..self
        }
    }

    pub open spec fn set_name(self, name: StringView) -> SubjectView {
        SubjectView {
            name: name,
            ..self
        }
    }

    pub open spec fn set_namespace(self, namespace: StringView) -> SubjectView {
        SubjectView {
            namespace: Option::Some(namespace),
            ..self
        }
    }

    pub open spec fn kind_field() -> nat {0}

    pub open spec fn name_field() -> nat {1}

    pub open spec fn namespace_field() -> nat {2}
}

impl Marshalable for SubjectView {
    open spec fn marshal(self) -> Value {
        Value::Object(
            Map::empty()
                .insert(Self::kind_field(), Value::String(self.kind))
                .insert(Self::name_field(), Value::String(self.name))
                .insert(Self::namespace_field(), if self.namespace.is_None() {Value::Null} else {
                    Value::String(self.namespace.get_Some_0())
                })
        )
    }

    open spec fn unmarshal(value: Value) -> Self {
        SubjectView {
            kind: value.get_Object_0()[Self::kind_field()].get_String_0(),
            name: value.get_Object_0()[Self::name_field()].get_String_0(),
            namespace: if value.get_Object_0()[Self::namespace_field()].is_Null() {Option::None} else {
                Option::Some(value.get_Object_0()[Self::namespace_field()].get_String_0())
            },
        }
    }

    proof fn marshal_preserves_integrity() {
        assert forall |o: Self| o == Self::unmarshal(#[trigger] o.marshal()) by {
        }
    }

}


}
