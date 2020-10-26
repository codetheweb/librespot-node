use neon::prelude::*;

pub struct AccessToken {
    pub token: String,
    pub scope: Vec<String>,
    pub expires_in: u32,
}

declare_types! {
    pub class JsAccessToken for AccessToken {
        init(mut cx) {
            let token: Handle<JsString> = cx.argument::<JsString>(0)?;
            let scopes: Handle<JsArray> = cx.argument::<JsArray>(1)?;
            let expires_in: Handle<JsNumber> = cx.argument::<JsNumber>(2)?;

            let vec = scopes.downcast::<JsArray>().unwrap().to_vec(&mut cx)?;
            let scope: Vec<String> = vec.iter().map(move |js_val| {
                let js_str = js_val.downcast::<JsString>().or_throw(&mut cx);

                match js_str {
                    Ok(s) => s.value(),
                    Err(_) => "<N/A>".to_string()
                }
            }).collect();

            Ok(AccessToken {
                token: token.value(),
                scope: scope,
                expires_in: expires_in.value() as u32,
            })
        }

        method getToken(mut cx) {
            let this = cx.this();
            let val = {
                let guard = cx.lock();
                let instance = this.borrow(&guard);

                instance.token.clone()
            };

            Ok(cx.string(val).upcast())
        }

        method getExpiry(mut cx) {
            let this = cx.this();
            let val = {
                let guard = cx.lock();
                let instance = this.borrow(&guard);

                instance.expires_in.clone()
            };

            Ok(cx.number(val).upcast())
        }

        method getScope(mut cx) {
            let this = cx.this();
            let scope = {
                let guard = cx.lock();
                let instance = this.borrow(&guard);

                instance.scope.clone()
            };

            let js_scopes = JsArray::new(&mut cx, scope.len() as u32);
            for (i, s) in scope.iter().enumerate() {
                let val = cx.string(s);
                let _ = js_scopes.set(&mut cx, i as u32, val);
            }

            Ok(js_scopes.upcast())
        }
    }
}