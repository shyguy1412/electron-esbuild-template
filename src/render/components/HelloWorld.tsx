import { h } from "preact";
import style from './HelloWorld.module.css';

type Props = {};

export function HelloWorld({ }: Props) {
    return <div id="content" class={style.center}>Hello World</div>
}