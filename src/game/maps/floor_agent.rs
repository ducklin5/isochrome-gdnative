use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::build)]
pub struct FloorAgent;

#[methods]
impl FloorAgent {
    fn new(_owner: &Node) -> Self {
        FloorAgent
    }

    fn build(builder: &ClassBuilder<Self>) {
        let arg0 = SignalArgument {
            name: "target",
            default: Variant::new(),
            export_info: ExportInfo::new(VariantType::Object),
            usage: PropertyUsage::DEFAULT,
        };

        builder.add_signal(Signal {
            name: "collision_changed",
            args: &[arg0],
        });
    }

    #[export]
    fn _ready(&self, _owner: &Node) {}

    #[export]
    fn notify(&self, owner: &Node, #[opt] target: Option<gdnative::Ref<Node>>) {
        let target_ref = if target.is_some() {
            target.unwrap()
        } else {
            owner.get_parent().unwrap()
        };

        let target_variant = Variant::from_object(target_ref);
        owner.emit_signal("collision_changed", &[target_variant]);
    }

    #[export]
    fn get_class(&self, _owner: &Node) -> Variant {
        return Variant::from_str("FloorAgent");
    }
}
