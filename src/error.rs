use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReconcileError {
    #[error("Kube Error: {0}")]
    KubeError(#[from] kube::Error),

    #[error("There are no exit nodes available to assign")]
    NoAvailableExitNodes,

    #[error("Exit Node being already assigned and its backoff time has not expired")]
    ExitNodeBackoff,

    #[error("There are no ports set on this LoadBalancer")]
    NoPortsSet,

    #[error("The provided cloud provisioner was not found in the cluster")]
    CloudProvisionerNotFound,
    #[error("The secret keys for the cloud provisioner were not found in the cluster")]
    CloudProvisionerSecretNotFound,

    #[error("The managed exit node spec does not have a password set")]
    ManagedExitNodeNoPasswordSet,

    #[error("The Secret could not be found in the resource's namespace")]
    SecretNotFound,

    #[error("The `auth` field is not set in the Secret intended for the password")]
    AuthFieldNotSet,

    #[error("The operator has encountered an unknown error, this is most likely a bug: {0}")]
    UnknownError(#[from] color_eyre::Report),
}
