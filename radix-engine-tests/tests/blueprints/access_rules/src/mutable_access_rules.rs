use scrypto::prelude::*;

#[blueprint]
mod mutable_access_rules_component {
    struct MutableAccessRulesComponent {}

    impl MutableAccessRulesComponent {
        pub fn new(roles: Roles) -> Global<MutableAccessRulesComponent> {
            Self {}
                .instantiate()
                .prepare_to_globalize()
                .define_roles(roles)
                .methods(methods! {
                    borrow_funds => ["borrow_funds_auth"];
                    deposit_funds => ["deposit_funds_auth"];
                    set_authority_rules => Public;
                    lock_authority => Public;
                })
                .globalize()
        }

        pub fn access_rules_function(component_address: ComponentAddress) {
            let component: Global<AnyComponent> = component_address.into();
            let _access_rules = component.access_rules();
        }

        pub fn set_authority_rules(&self, authority: String, rule: AccessRule) {
            let access_rules = Runtime::access_rules();
            access_rules.set_authority_rule(authority.as_str(), rule);
        }

        pub fn lock_authority(&self, authority: String) {
            let access_rules = Runtime::access_rules();
            access_rules.set_authority_mutability(authority.as_str(), vec![]);
        }

        // The methods that the access rules will be added to
        pub fn borrow_funds(&self) {}

        pub fn deposit_funds(&self) {}
    }
}
