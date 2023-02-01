import * as wasm from "../pkg/hi_push";
import {ResponseError} from "../pkg/hi_push";

let cli = new wasm.Client("http://127.0.0.1:4523/m1/2238314-0-default/", "", "")

console.log("213")

cli.registerToken(new wasm.RegisterTokenParams("", "", "")).then(r => {
    console.log(r)
}).catch((reason) => {
    if (reason instanceof ResponseError){
        console.log("error")
        console.log(reason)
        console.log(reason.code)
    }
})

cli.debug().then(r => {
    console.log(r)
}).catch(reason => {
    console.log(reason)
})