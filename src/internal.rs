extern crate imgui;

use crate::widget;

use crate::vec2;

pub struct NodeBuilder(Node);

impl NodeBuilder {
    pub fn new(id: String, class: String, label: String) -> Self {
        Self(Node {
            address: imgui::ImString::from(format!("{}:{}", class, id)),
            class: imgui::ImString::from(class.clone()),
            id: imgui::ImString::from(id.clone()),
            label: imgui::ImString::from(label),
            input_pins: Vec::new(),
            output_pins: Vec::new(),
            position: [0.0, 0.0],
            active: false,
            selected_pin: None,
        })
    }

    pub fn add_input_pin(mut self, class: String, label: String) -> Self {
        self.0.input_pins.push(Pin {
            address: imgui::ImString::from(format!("{}:in:{}", self.0.address, class)),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
        });
        self
    }

    pub fn add_output_pin(mut self, class: String, label: String) -> Self {
        self.0.output_pins.push(Pin {
            address: imgui::ImString::from(format!("{}:out:{}", self.0.address, class)),
            class: imgui::ImString::from(class),
            label: imgui::ImString::from(label),
        });
        self
    }

    pub fn build(self) -> Node {
        self.0
    }
}

pub struct Node {
    pub address: imgui::ImString,
    class: imgui::ImString,
    id: imgui::ImString,
    label: imgui::ImString,
    input_pins: Vec<Pin>,
    output_pins: Vec<Pin>,
    pub position: [f32; 2],
    pub active: bool,
    pub selected_pin: Option<imgui::ImString>,
}

pub struct Pin {
    address: imgui::ImString,
    class: imgui::ImString,
    label: imgui::ImString,
}

impl Node {
    pub fn draw(&mut self, ui: &imgui::Ui<'_>, offset: [f32; 2]) {
        let mut pin_group = widget::pin_group::PinGroup::new();

        for input_pin in self.input_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&input_pin.address, &input_pin.label)
                    .orientation(widget::pin::Orientation::Left),
            );
        }

        for output_pin in self.output_pins.iter() {
            pin_group = pin_group.add_pin(
                widget::pin::Pin::new(&output_pin.address, &output_pin.label)
                    .orientation(widget::pin::Orientation::Right),
            );
        }

        widget::node::Node::new(&self.address)
            .position(vec2::sum(&[self.position, offset]))
            .add_component(widget::node::Component::Label(widget::label::Label::new(
                &self.label,
            )))
            .add_component(widget::node::Component::Space(5.0))
            .add_component(widget::node::Component::PinGroup {
                pin_group,
                selected_pin: &mut self.selected_pin,
            })
            .add_component(widget::node::Component::Space(10.0))
            .build(ui);
        self.active = ui.is_item_active();
        unsafe {
            imgui::sys::igSetItemAllowOverlap();
        }

        if self.selected_pin.is_some() {
            println!("{} {}", self.address, self.selected_pin.as_ref().unwrap());
        }
    }
}
