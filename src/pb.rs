
pub mod my {
    pub mod solana {
        pub mod transfers {
            pub mod v1 {
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct Transfer {
                    #[prost(string, tag = "1")]
                    pub from: String,

                    #[prost(string, tag = "2")]
                    pub to: String,

                    #[prost(uint64, tag = "3")]
                    pub amount: u64,
                }

                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct Transfers {
                    #[prost(message, repeated, tag = "1")]
                    pub transfers: Vec<Transfer>,
                }
            }
        }
    }
}
