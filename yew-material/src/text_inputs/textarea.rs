use super::set_on_input_handler;
use crate::text_inputs::validity_state::ValidityStateJS;
use crate::text_inputs::{TextFieldType, ValidityState, ValidityTransform};
use crate::{to_option, to_option_string};
use gloo::events::EventListener;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Node;
pub use web_sys::ValidityState as NativeValidityState;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-textarea.js")]
extern "C" {
    #[derive(Debug)]
    #[wasm_bindgen(extends = Node)]
    type TextArea;

    #[wasm_bindgen(getter, static_method_of = TextArea)]
    fn _dummy_loader() -> JsValue;

    #[wasm_bindgen(method, setter = validityTransform)]
    fn set_validity_transform(
        this: &TextArea,
        val: &Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>,
    );

    #[wasm_bindgen(method, setter)]
    fn set_type(this: &TextArea, val: &JsValue);

    #[wasm_bindgen(method, getter)]
    fn value(this: &TextArea) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &TextArea, val: &JsValue);
}

loader_hack!(TextArea);

/// The `mwc-textarea` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/textarea)
pub struct MatTextArea {
    props: TextAreaProps,
    node_ref: NodeRef,
    validity_transform_closure:
        Option<Closure<dyn Fn(String, NativeValidityState) -> ValidityStateJS>>,
    input_listener: Option<EventListener>,
}

/// Type for [`TextAreaProps::char_counter`].
///
/// Equivalent to `type TextAreaCharCounter = 'external'|'internal';` Typescript
/// type.
#[derive(Clone)]
pub enum TextAreaCharCounter {
    Internal,
    External,
}

impl ToString for TextAreaCharCounter {
    fn to_string(&self) -> String {
        use TextAreaCharCounter::*;
        match self {
            Internal => "internal",
            External => "external",
        }
        .to_string()
    }
}

/// Props for [`MatTextArea`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/checkbox#propertiesattributes)
#[derive(Properties, Clone)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub rows: Option<i64>,
    #[prop_or_default]
    pub cols: Option<i64>,
    #[prop_or_default]
    pub value: String,
    #[prop_or(TextFieldType::Text)]
    pub field_type: TextFieldType,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub placeholder: String,
    #[prop_or_default]
    pub icon: String,
    #[prop_or_default]
    pub icon_trailing: String,
    #[prop_or_default]
    pub disabled: bool,
    /// For boolean value `true`, `TextAreaCharCounter::External` is to be used.
    /// Boolean value `false` results in character counter not being shown so
    /// `None` should be used
    #[prop_or_default]
    pub char_counter: Option<TextAreaCharCounter>,
    #[prop_or_default]
    pub outlined: bool,
    #[prop_or_default]
    pub helper: String,
    #[prop_or_default]
    pub helper_persistent: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub max_length: String,
    #[prop_or_default]
    pub validation_message: String,
    /// Type: `number | string` so I'll leave it as a string
    #[prop_or_default]
    pub min: String,
    /// Type: `number | string`  so I'll leave it as a string
    #[prop_or_default]
    pub max: String,
    #[prop_or_default]
    pub size: Option<i64>, // --|
    #[prop_or_default] //   | -- What you doing step size
    pub step: Option<i64>, // --|
    #[prop_or_default]
    pub auto_validate: bool,
    #[prop_or_default]
    pub validity_transform: Option<ValidityTransform>,
    #[prop_or_default]
    pub validate_on_initial_render: bool,
    #[prop_or_default]
    pub oninput: Callback<InputData>,
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub fullwidth: bool,
}

impl Component for MatTextArea {
    type Message = ();
    type Properties = TextAreaProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TextArea::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            validity_transform_closure: None,
            input_listener: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-textarea
                rows?=self.props.rows
                cols?=self.props.cols
                label?=to_option_string(&self.props.label)
                placeholder?=to_option_string(&self.props.placeholder)
                icon?=to_option_string(&self.props.icon)
                iconTrailing?=to_option_string(&self.props.icon_trailing)
                disabled=self.props.disabled
                charCounter?=self.props.char_counter.as_ref().map(|it| it.to_string())
                outlined?=to_option(self.props.outlined)
                helper?=to_option_string(&self.props.helper)
                helperPersistent?=to_option(self.props.helper_persistent)
                required=self.props.required
                maxLength?=to_option_string(&self.props.max_length)
                validationMessage?=to_option_string(&self.props.validation_message)
                min?=to_option_string(&self.props.min)
                max?=to_option_string(&self.props.max)
                size?=self.props.size
                step?=self.props.step
                autoValidate?=to_option(self.props.auto_validate)
                validateOnInitialRender?=to_option(self.props.validate_on_initial_render)
                name?=to_option_string(&self.props.name)
                ref=self.node_ref.clone()
                style=if self.props.fullwidth { "width: 100%" } else { "" }  
            ></mwc-textarea>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        let element = self.node_ref.cast::<TextArea>().unwrap();
        element.set_type(&JsValue::from(&self.props.field_type.to_string()));
        element.set_value(&JsValue::from(&self.props.value));

        if first_render {
            self.input_listener = Some(set_on_input_handler(
                &self.node_ref,
                self.props.oninput.clone(),
                |(input_event, detail)| {
                    InputData {
                        value: detail
                            .unchecked_into::<MatTextAreaInputEvent>()
                            .target()
                            .value(),
                        event: input_event,
                    }
                },
            ));

            let this = self.node_ref.cast::<TextArea>().unwrap();
            if let Some(transform) = self.props.validity_transform.clone() {
                self.validity_transform_closure = Some(Closure::wrap(Box::new(
                    move |s: String, v: NativeValidityState| -> ValidityStateJS {
                        transform.0(s, v).into()
                    },
                )
                    as Box<dyn Fn(String, NativeValidityState) -> ValidityStateJS>));
                this.set_validity_transform(&self.validity_transform_closure.as_ref().unwrap());
            }
        }
    }
}

impl MatTextArea {
    pub fn validity_transform<F: Fn(String, NativeValidityState) -> ValidityState + 'static>(
        func: F,
    ) -> ValidityTransform {
        ValidityTransform::new(func)
    }
}

#[wasm_bindgen]
extern "C" {
    type MatTextAreaInputEvent;

    #[wasm_bindgen(method, getter)]
    fn target(this: &MatTextAreaInputEvent) -> TextArea;
}
