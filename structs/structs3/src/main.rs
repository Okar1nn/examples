#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    // fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Self {
        // if weight_in_grams <= 0 {
        //     panic!("Can not ship a weightless package.")
        // } else {
        //     Package {
        //         sender_country,
        //         recipient_country,
        //         weight_in_grams,
        //     }
        // }

        assert!(weight_in_grams > 0, "Can not ship a weightless package.");

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    const fn get_fees(&self, cents_per_gram: i32) -> i32 {
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = "Spain".to_owned();
        let recipient_country = "Austria".to_owned();

        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let sender_country = "Spain".to_owned();
        let recipient_country = "Russia".to_owned();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = "Canada".to_owned();
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        // assert!(!package.is_international());
        assert_ne!(package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = "Spain".to_owned();
        let recipient_country = "Spain".to_owned();

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
