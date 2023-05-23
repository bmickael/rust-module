import("./xterm.css").catch(console.error);
import('./pkg').then((mod) => {
    let ar = new Array();
    ar.push(42);
    ar.push(11);
    console.log(ar);
    console.log(mod.process_array(ar));
    console.log(ar);
    console.log(mod.process_array_2(ar));
    console.log(ar);
    let arr = mod.GetJsArray();
    console.log(arr);
    let arr2 = mod.GetJsObject();
    console.log(arr2);
    //mod.call_js_function();
    mod.call_js2_function();

    
}).catch(console.error);
