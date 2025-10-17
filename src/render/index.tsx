import './index.css';
import 'normalize.css';
import { h, render } from "preact";
import { HelloWorld } from "./components/HelloWorld";

type Props = {};

function Index({ }: Props) {
    
    return <HelloWorld></HelloWorld>;
}
__module_bridge_init.then(() =>
    render(<Index></Index>, document.body)
);
