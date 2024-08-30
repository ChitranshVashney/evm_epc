use crate::types::candid::{
    Block, BlockTag, MultiRpcResult, ProviderError, RpcConfig, RpcError, RpcServices,
};

mod max_expected_too_few_cycles_error {
    use super::*;
    use crate::max_expected_too_few_cycles_error;
    use crate::types::candid::{RpcApi, RpcService};

    #[test]
    fn should_get_max_expected_too_few_cycles_for_custom_rpc_service() {
        let result: MultiRpcResult<()> = MultiRpcResult::Inconsistent(vec![
            (
                RpcService::Custom(RpcApi {
                    url: "https://eth.llamarpc.com".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 701_433_600,
                    received: 350_729_600,
                })),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://ethereum-rpc.publicnode.com".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 863_894_400,
                    received: 350_729_600,
                })),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://rpc.ankr.com/eth".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 893_894_400,
                    received: 350_729_600,
                })),
            ),
        ]);

        let max_too_few_cycles = max_expected_too_few_cycles_error(&result);

        assert_eq!(max_too_few_cycles, Some(893_894_400));
    }

    #[test]
    fn should_return_none_for_no_too_few_cycles_errors() {
        let result: MultiRpcResult<()> = MultiRpcResult::Inconsistent(vec![
            (
                RpcService::Custom(RpcApi {
                    url: "https://eth.llamarpc.com".to_string(),
                    headers: None,
                }),
                Ok(()),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://ethereum-rpc.publicnode.com".to_string(),
                    headers: None,
                }),
                Ok(()),
            ),
        ]);

        let max_too_few_cycles = max_expected_too_few_cycles_error(&result);

        assert_eq!(max_too_few_cycles, None);
    }

    #[test]
    fn should_handle_mixed_errors_with_too_few_cycles() {
        let result: MultiRpcResult<()> = MultiRpcResult::Inconsistent(vec![
            (
                RpcService::Custom(RpcApi {
                    url: "https://eth.llamarpc.com".to_string(),
                    headers: None,
                }),
                Ok(()),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://rpc.ankr.com/eth".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 500_000_000,
                    received: 250_000_000,
                })),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://ethereum-rpc.publicnode.com".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 750_000_000,
                    received: 250_000_000,
                })),
            ),
        ]);

        let max_too_few_cycles = max_expected_too_few_cycles_error(&result);

        assert_eq!(max_too_few_cycles, Some(750_000_000));
    }

    #[test]
    fn should_return_none_for_consistent_successful_results() {
        let result: MultiRpcResult<()> = MultiRpcResult::Consistent(Ok(()));

        let max_too_few_cycles = max_expected_too_few_cycles_error(&result);

        assert_eq!(max_too_few_cycles, None);
    }

    #[test]
    fn should_return_max_expected_too_few_cycles_for_mixed_rpc_services() {
        let result: MultiRpcResult<()> = MultiRpcResult::Inconsistent(vec![
            (
                RpcService::Custom(RpcApi {
                    url: "https://rpc.ankr.com/eth".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 500_000_000,
                    received: 250_000_000,
                })),
            ),
            (
                RpcService::Custom(RpcApi {
                    url: "https://rpc.ankr.com/eth".to_string(),
                    headers: None,
                }),
                Err(RpcError::ProviderError(ProviderError::TooFewCycles {
                    expected: 1_000_000_000,
                    received: 250_000_000,
                })),
            ),
        ]);

        let max_too_few_cycles = max_expected_too_few_cycles_error(&result);

        assert_eq!(max_too_few_cycles, Some(1_000_000_000));
    }
}
