use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    // Step 1: Create a new CancellationToken
    let token = CancellationToken::new();

    // Step 2: Clone the token for use in another task
    let cloned_token = token.clone();

    // Task 1 - Wait for token cancellation or a long time
    let task1_handle = tokio::spawn(async move {
        tokio::select! {
            // Step 3: Using cloned token to listen to cancellation requests
            _ = cloned_token.cancelled() => {
                println!("cloned token cancelled");
                // The token was cancelled, task can shut down
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                // Long work has completed
            }
        }
    });

    let clone_token2 = token.clone();
    let task1_handle2 = tokio::spawn(async move {
        tokio::select! {
            // Step 3: Using cloned token to listen to cancellation requests
            _ = clone_token2.cancelled() => {
                // The token was cancelled, task can shut down
                println!("cloned token2 cancelled");
            }
            _ = tokio::time::sleep(std::time::Duration::from_secs(9999)) => {
                // Long work has completed
            }
        }
    });

    // Task 2 - Cancel the original token after a small delay
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        // Step 4: Cancel the original or clonned token to notify other tasks about shutting down gracefully
        token.cancel();
    });

    // Wait for tasks to complete
    let _res = tokio::join!(task1_handle, task1_handle2);
}
