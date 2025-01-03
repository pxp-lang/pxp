use pxp_bytestring::{ByteStr, ByteString};

use crate::{
    DocBlock, DocBlockDeprecatedTag, DocBlockExtendsTag, DocBlockGenericTag, DocBlockImplementsTag,
    DocBlockMethodTag, DocBlockNode, DocBlockParamTag, DocBlockPropertyTag, DocBlockReturnTag,
    DocBlockTag, DocBlockTagNode, DocBlockTemplateTag, DocBlockTextNode, DocBlockUsesTag,
    DocBlockVarTag,
};

pub struct DocBlockTagCollection<'a> {
    tags: Vec<&'a DocBlockTagNode>,
}

impl<'a> DocBlockTagCollection<'a> {
    pub fn collect(&self) -> Vec<&DocBlockTag> {
        self.tags.iter().map(|t| t.tag()).collect()
    }

    pub fn get_param_tags(&self) -> Vec<&DocBlockParamTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_param())
            .collect()
    }

    pub fn get_return_tags(&self) -> Vec<&DocBlockReturnTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_return())
            .collect()
    }

    pub fn get_method_tags(&self) -> Vec<&DocBlockMethodTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_method())
            .collect()
    }

    pub fn get_property_tags(&self) -> Vec<&DocBlockPropertyTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_property())
            .collect()
    }

    pub fn get_var_tags(&self) -> Vec<&DocBlockVarTag> {
        self.tags.iter().filter_map(|t| t.tag().as_var()).collect()
    }

    pub fn get_template_tags(&self) -> Vec<&DocBlockTemplateTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_template())
            .collect()
    }

    pub fn get_extends_tags(&self) -> Vec<&DocBlockExtendsTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_extends())
            .collect()
    }

    pub fn get_implements_tags(&self) -> Vec<&DocBlockImplementsTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_implements())
            .collect()
    }

    pub fn get_uses_tags(&self) -> Vec<&DocBlockUsesTag> {
        self.tags.iter().filter_map(|t| t.tag().as_uses()).collect()
    }

    pub fn get_deprecated_tags(&self) -> Vec<&DocBlockDeprecatedTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_deprecated())
            .collect()
    }

    pub fn get_generic_tags(&self) -> Vec<&DocBlockGenericTag> {
        self.tags
            .iter()
            .filter_map(|t| t.tag().as_generic())
            .collect()
    }
}

impl DocBlockTag {
    pub fn as_var(&self) -> Option<&DocBlockVarTag> {
        match self {
            DocBlockTag::Var(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_template(&self) -> Option<&DocBlockTemplateTag> {
        match self {
            DocBlockTag::Template(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_extends(&self) -> Option<&DocBlockExtendsTag> {
        match self {
            DocBlockTag::Extends(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_implements(&self) -> Option<&DocBlockImplementsTag> {
        match self {
            DocBlockTag::Implements(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_uses(&self) -> Option<&DocBlockUsesTag> {
        match self {
            DocBlockTag::Uses(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_deprecated(&self) -> Option<&DocBlockDeprecatedTag> {
        match self {
            DocBlockTag::Deprecated(node) => Some(node),
            _ => None,
        }
    }

    pub fn as_generic(&self) -> Option<&DocBlockGenericTag> {
        match self {
            DocBlockTag::Generic(node) => Some(node),
            _ => None,
        }
    }

    pub fn as_param(&self) -> Option<&DocBlockParamTag> {
        match self {
            DocBlockTag::Param(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_method(&self) -> Option<&DocBlockMethodTag> {
        match self {
            DocBlockTag::Method(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_property(&self) -> Option<&DocBlockPropertyTag> {
        match self {
            DocBlockTag::Property(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_return(&self) -> Option<&DocBlockReturnTag> {
        match self {
            DocBlockTag::Return(tag) => Some(tag),
            _ => None,
        }
    }
}

impl DocBlock {
    pub fn nodes(&self) -> &[DocBlockNode] {
        &self.nodes
    }

    pub fn tags(&self) -> DocBlockTagCollection {
        DocBlockTagCollection {
            tags: self.tag_nodes(),
        }
    }

    pub fn tag_nodes(&self) -> Vec<&DocBlockTagNode> {
        self.nodes
            .iter()
            .filter_map(DocBlockNode::as_tag)
            .collect::<Vec<_>>()
    }

    pub fn text_nodes(&self) -> Vec<&DocBlockTextNode> {
        self.nodes
            .iter()
            .filter_map(DocBlockNode::as_text)
            .collect::<Vec<_>>()
    }

    pub fn text(&self) -> Vec<&ByteStr> {
        self.text_nodes()
            .into_iter()
            .map(DocBlockTextNode::text)
            .collect()
    }
}

impl DocBlockTextNode {
    pub fn text(&self) -> &ByteStr {
        self.content.as_ref()
    }
}

impl DocBlockTagNode {
    pub fn tag(&self) -> &DocBlockTag {
        &self.tag
    }
}

impl DocBlockNode {
    pub fn is_tag(&self) -> bool {
        matches!(self, DocBlockNode::Tag(_))
    }

    pub fn is_text(&self) -> bool {
        matches!(self, DocBlockNode::Text(_))
    }

    pub fn as_tag(&self) -> Option<&DocBlockTagNode> {
        match self {
            DocBlockNode::Tag(tag) => Some(tag),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&DocBlockTextNode> {
        match self {
            DocBlockNode::Text(node) => Some(node),
            _ => None,
        }
    }
}
