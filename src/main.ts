import "./styles.css";
import WalletTray from "./WalletTray.svelte";
import TransactionManager from "./TransactionManager.svelte";

const walletTrayApp = new WalletTray({
 target: document.getElementById("walletTray"),
});

const transactionManagerApp = new TransactionManager({
 target: document.getElementById("transactionManager"),
});

export { walletTray, transactionManager };
