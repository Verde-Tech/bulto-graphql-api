use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protocol/api_gateway.proto")?;
    tonic_build::compile_protos("../user_management/protocol/user_management.proto")?;
    tonic_build::compile_protos("../financial_transactions/protocol/financial_transactions.proto")?;
    tonic_build::compile_protos("../credit_scoring/protocol/credit_scoring.proto")?;
    tonic_build::compile_protos("../lending/protocol/lending.proto")?;
    tonic_build::compile_protos("../card_integration/protocol/card_integration.proto")?;
    tonic_build::compile_protos("../point_of_sale/protocol/point_of_sale.proto")?;
    tonic_build::compile_protos("../mobile_money/protocol/mobile_money.proto")?;
    tonic_build::compile_protos("../crm/protocol/crm.proto")?;
    tonic_build::compile_protos("../ai_assistant/protocol/ai_assistant.proto")?;
    tonic_build::compile_protos("../fraud_detection/protocol/fraud_detection.proto")?;
    tonic_build::compile_protos("../notification/protocol/notification.proto")?;
    tonic_build::compile_protos("../reporting_analytics/protocol/reporting_analytics.proto")?;
    tonic_build::compile_protos("../database_service/protocol/database_service.proto")?;
    tonic_build::compile_protos("../authentication_authorization/protocol/authentication_authorization.proto")?;
    tonic_build::compile_protos("../load_balancing/protocol/load_balancing.proto")?;
    tonic_build::compile_protos("../cache_management/protocol/cache_management.proto")?;
    tonic_build::compile_protos("../logging_monitoring/protocol/logging_monitoring.proto")?;
    tonic_build::compile_protos("../data_synchronization/protocol/data_synchronization.proto")?;
    tonic_build::compile_protos("../backup_recovery/protocol/backup_recovery.proto")?;
    tonic_build::compile_protos("../message_queuing/protocol/message_queuing.proto")?;
    tonic_build::compile_protos("../configuration_management/protocol/configuration_management.proto")?;
    tonic_build::compile_protos("../security/protocol/security.proto")?;
    tonic_build::compile_protos("../service_discovery/protocol/service_discovery.proto")?;
    tonic_build::compile_protos("../microservices_deployment/protocol/microservices_deployment.proto")?;

    Ok(())
}
