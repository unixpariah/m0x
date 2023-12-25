import "./styles.css";
import WalletTray from "./WalletTray.svelte";

const app = new WalletTray({
 target: document.getElementById("app"),
});

export default app;
