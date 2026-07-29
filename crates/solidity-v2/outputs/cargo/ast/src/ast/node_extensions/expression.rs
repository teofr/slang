use std::ops::Range;

use super::super::{ElementaryType, Expression, Type};

impl Expression {
    /// Returns the type assigned to this expression by the typing pass, or
    /// `None` when no type was recorded for the underlying node.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            Expression::AssignmentExpression(expression) => expression.get_type(),
            Expression::ConditionalExpression(expression) => expression.get_type(),
            Expression::OrExpression(expression) => expression.get_type(),
            Expression::AndExpression(expression) => expression.get_type(),
            Expression::EqualityExpression(expression) => expression.get_type(),
            Expression::InequalityExpression(expression) => expression.get_type(),
            Expression::BitwiseOrExpression(expression) => expression.get_type(),
            Expression::BitwiseXorExpression(expression) => expression.get_type(),
            Expression::BitwiseAndExpression(expression) => expression.get_type(),
            Expression::ShiftExpression(expression) => expression.get_type(),
            Expression::AdditiveExpression(expression) => expression.get_type(),
            Expression::MultiplicativeExpression(expression) => expression.get_type(),
            Expression::ExponentiationExpression(expression) => expression.get_type(),
            Expression::PostfixExpression(expression) => expression.get_type(),
            Expression::PrefixExpression(expression) => expression.get_type(),
            Expression::FunctionCallExpression(expression) => expression.get_type(),
            Expression::CallOptionsExpression(expression) => expression.get_type(),
            Expression::MemberAccessExpression(expression) => expression.get_type(),
            Expression::IndexAccessExpression(expression) => expression.get_type(),
            Expression::NewExpression(expression) => expression.get_type(),
            Expression::TupleExpression(expression) => expression.get_type(),
            Expression::TypeExpression(expression) => expression.get_type(),
            Expression::ArrayExpression(expression) => expression.get_type(),
            Expression::HexNumberExpression(expression) => expression.get_type(),
            Expression::DecimalNumberExpression(expression) => expression.get_type(),
            Expression::StringExpression(expression) => expression.get_type(),
            Expression::ElementaryType(expression) => expression.get_type(),
            Expression::PayableKeyword(expression) => expression.get_type(),
            Expression::ThisKeyword(expression) => expression.get_type(),
            Expression::SuperKeyword(expression) => expression.get_type(),
            Expression::TrueKeyword(expression) => expression.get_type(),
            Expression::FalseKeyword(expression) => expression.get_type(),
            Expression::Identifier(expression) => expression.get_type(),
        }
    }

    /// Returns the byte range this expression spans in its source file.
    ///
    /// Unlike the individual node structs (which return a borrowed range), this
    /// returns an owned range: most variants clone the underlying node's range,
    /// but the string-literal variant spans a sequence of tokens and computes
    /// its range on the fly.
    pub fn get_text_range(&self) -> Range<usize> {
        match self {
            Expression::AssignmentExpression(expression) => expression.get_text_range().clone(),
            Expression::ConditionalExpression(expression) => expression.get_text_range().clone(),
            Expression::OrExpression(expression) => expression.get_text_range().clone(),
            Expression::AndExpression(expression) => expression.get_text_range().clone(),
            Expression::EqualityExpression(expression) => expression.get_text_range().clone(),
            Expression::InequalityExpression(expression) => expression.get_text_range().clone(),
            Expression::BitwiseOrExpression(expression) => expression.get_text_range().clone(),
            Expression::BitwiseXorExpression(expression) => expression.get_text_range().clone(),
            Expression::BitwiseAndExpression(expression) => expression.get_text_range().clone(),
            Expression::ShiftExpression(expression) => expression.get_text_range().clone(),
            Expression::AdditiveExpression(expression) => expression.get_text_range().clone(),
            Expression::MultiplicativeExpression(expression) => expression.get_text_range().clone(),
            Expression::ExponentiationExpression(expression) => expression.get_text_range().clone(),
            Expression::PostfixExpression(expression) => expression.get_text_range().clone(),
            Expression::PrefixExpression(expression) => expression.get_text_range().clone(),
            Expression::FunctionCallExpression(expression) => expression.get_text_range().clone(),
            Expression::CallOptionsExpression(expression) => expression.get_text_range().clone(),
            Expression::MemberAccessExpression(expression) => expression.get_text_range().clone(),
            Expression::IndexAccessExpression(expression) => expression.get_text_range().clone(),
            Expression::NewExpression(expression) => expression.get_text_range().clone(),
            Expression::TupleExpression(expression) => expression.get_text_range().clone(),
            Expression::TypeExpression(expression) => expression.get_text_range().clone(),
            Expression::ArrayExpression(expression) => expression.get_text_range().clone(),
            Expression::HexNumberExpression(expression) => expression.get_text_range().clone(),
            Expression::DecimalNumberExpression(expression) => expression.get_text_range().clone(),
            Expression::StringExpression(expression) => expression.get_text_range(),
            Expression::ElementaryType(expression) => expression.get_text_range(),
            Expression::PayableKeyword(expression) => expression.get_text_range().clone(),
            Expression::ThisKeyword(expression) => expression.get_text_range().clone(),
            Expression::SuperKeyword(expression) => expression.get_text_range().clone(),
            Expression::TrueKeyword(expression) => expression.get_text_range().clone(),
            Expression::FalseKeyword(expression) => expression.get_text_range().clone(),
            Expression::Identifier(expression) => expression.get_text_range().clone(),
        }
    }
}

impl ElementaryType {
    /// Returns the type assigned to this elementary type by the typing pass,
    /// dispatched to the underlying keyword/terminal node.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            ElementaryType::BoolKeyword(keyword) => keyword.get_type(),
            ElementaryType::StringKeyword(keyword) => keyword.get_type(),
            ElementaryType::AddressType(address_type) => address_type.get_type(),
            ElementaryType::BytesKeyword(keyword) => keyword.get_type(),
            ElementaryType::IntKeyword(keyword) => keyword.get_type(),
            ElementaryType::UintKeyword(keyword) => keyword.get_type(),
            ElementaryType::FixedKeyword(keyword) => keyword.get_type(),
            ElementaryType::UfixedKeyword(keyword) => keyword.get_type(),
        }
    }

    /// Returns the byte range this elementary type spans, dispatched to the
    /// underlying keyword/terminal node.
    pub fn get_text_range(&self) -> Range<usize> {
        match self {
            ElementaryType::BoolKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::StringKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::AddressType(address_type) => address_type.get_text_range().clone(),
            ElementaryType::BytesKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::IntKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::UintKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::FixedKeyword(keyword) => keyword.get_text_range().clone(),
            ElementaryType::UfixedKeyword(keyword) => keyword.get_text_range().clone(),
        }
    }
}
