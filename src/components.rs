use crate::fin_math::{round2, Loan, Snapshot};
use crate::log;
use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Number")]
    type JsNumber;

    #[wasm_bindgen(constructor, js_class = "Number")]
    fn new(x: f64) -> JsNumber;

    #[wasm_bindgen(method, js_name = "toLocaleString")]
    fn to_locale_string(this: &JsNumber) -> String;
}

fn display_dollars(x: f64) -> String {
    format!("${}", JsNumber::new(x).to_locale_string())
}

#[component]
fn InputNumber<S: ToString, F: Fn(f64) + 'static>(
    prompt: S,
    setter: F,
    #[prop(optional)] init_value: f64,
) -> impl IntoView {
    let prompt = prompt.to_string();
    view! {
        <span class="font-medium">{prompt}</span>
        <input class="border p-1 m-1 rounded border-slate"
            type="number"
            on:input=move |evt| {
                setter(event_target_value(&evt).parse().unwrap_or(0.0))
            }
        value=init_value/>
    }
}

#[component]
fn DisplayTime<F: Fn() -> i32 + 'static>(days: F) -> impl IntoView {
    move || {
        let mut days = days();
        let years = days / 365;
        days = days - years * 365;
        let months = days / 30;
        days = days - months * 30;
        let mut display = String::new();
        if years > 0 {
            display.push_str(&format!("{years} years ")[..]);
        }
        if months > 0 {
            display.push_str(&format!("{months} months ")[..]);
        }
        display.push_str(&format!("{days} days")[..]);
        view! {
          <p>Total time: {display}</p>
        }
    }
}

#[component]
fn PaymentResults<F: Fn() -> Loan + 'static>(loan: F) -> impl IntoView {
    let res = move || loan().run_until_done();
    move || match res() {
        Ok(Snapshot {
            human_time,
            interest,
            ..
        }) => {
            view! {
                <p class="py-1">
                    <span class="font-medium pr-2">Total time:</span> {human_time}
                </p>
                <p class="py-1">
                    <span class="font-medium pr-2">Interest paid:</span> {display_dollars(interest)}
                </p>
            }
        }
        _ => view! {
          <p class="py-1">Invalid monthly payment!</p>
          <p class="py-1">"i.e. you'd never pay it off..."</p>
        },
    }
}

#[component]
fn DisplaySnapshots(snapshots: ReadSignal<Vec<Snapshot>>) -> impl IntoView {
    let th_class = "border border-slate-300 p-2";
    let td_class = "border border-slate-300 p-2";
    move || {
        let snapshots = snapshots.get();
        if snapshots.is_empty() {
            view! {
                <>
                    <p class="py-2">
                        Take a snapshot to view multiple payment plans
                    </p>
                </>
            }
        } else {
            view! {
                <>
                    <table class="border rounded border-collapse my-2 py-2">
                        <thead>
                            <tr>
                                <th class={th_class}>Amount</th>
                                <th class={th_class}>APR</th>
                                <th class={th_class}>Payment</th>
                                <th class={th_class}>Every</th>
                                <th class={th_class}>Interest</th>
                                <th class={th_class}>Time</th>
                            </tr>
                        </thead>
                        <tbody>
                            <For
                                each=move || snapshots.clone()
                                key=|s| s.human_time.clone()
                                let:s>
                                <tr class="odd:bg-white even:bg-slate-100">
                                    <td class={td_class}>{display_dollars(s.amount)}</td>
                                    <td class={td_class}>{round2(s.apr * 100.0)}%</td>
                                    <td class={td_class}>{display_dollars(s.payment)}</td>
                                    <td class={td_class}>{s.every}</td>
                                    <td class={td_class}>{display_dollars(s.interest)}</td>
                                    <td class={td_class}>{s.human_time}</td>
                                </tr>
                            </For>
                        </tbody>
                    </table>
                </>
            }
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (amount, set_amount) = create_signal(43524.71);
    let (year_rate, set_year_rate) = create_signal(7.50);
    let (compounding, set_compounding) = create_signal("daily".to_string());
    let (payment, set_payment) = create_signal(730.99);
    let (payday, set_payday) = create_signal(30);
    let loan = move || Loan::new(amount(), year_rate(), compounding(), payment(), payday());
    let (snapshots, set_snapshots) = create_signal(vec![]);

    view! {
        <main class="container mx-auto w-2/3">
        <h1 class="text-3xl font-bold py-1">Simple Loan Calculator</h1>
        <p class="py-1">
            <b>Note:</b> this is a <i>simple</i> calculator.
            The results may vary from actual loan paperwork.
            This can happen because "pay every 30 days" is not the same as "pay every month,"
            because some months are shorter and some are longer.
            Also this calculator does not take into account leap years.
        </p>
        <p class="py-1">
            That being said, a real loan will likely fall between
            what this calculator gets for "\"pay every 30 days\"" and
            "\"pay every 31 days.\"" I hope this is useful.
        </p>
        <hr class="my-2 border"/>
        <p class="py-2">
            <InputNumber
                prompt="Amount owed $"
                setter=set_amount
                init_value=43524.71 />
        </p>
        <p class="py-2">
            <InputNumber
                prompt="Annual Percentage Rate: "
                setter=set_year_rate
                init_value=7.50 />%
            <span class="font-medium pr-1">compounding:</span>
            <select class="border rounded p-1 border-slate"
                on:input=move |evt| {
                    set_compounding(event_target_value(&evt));
                }>
                <option value="daily">Daily</option>
                <option value="monthly">Monthly</option>
                <option value="yearly">Yearly</option>
            </select>
        </p>
        <p class="py-2">
            <InputNumber
                prompt="Payment of"
                setter=set_payment
                init_value=730.99 />
            <span class="font-medium">every:</span>
            <input class="border p-1 m-1 rounded border-slate"
                type="number"
                on:input=move |evt| {
                    set_payday(event_target_value(&evt).parse().unwrap_or(30))
                }
                value=30/>
            day(s)
        </p>
        <hr class="my-2 border"/>
        <p class="py-1">Calculating based on a loan of {move || display_dollars(amount())} at {year_rate}% APR</p>
        <PaymentResults loan=loan />
        <p>
            <button class="border border-slate-300 rounded p-1.5 mr-1 bg-slate-100"
                on:click=move |_| {
                    let res = loan().run_until_done();
                    match res {
                        Ok(s) => {
                            let mut snapshots = snapshots();
                            snapshots.push(s);
                            set_snapshots(snapshots)
                        },
                        _ => {
                            log!("not adding invalid snapshot!")
                        }
                    }
                }>
                Take Snapshot
            </button>
            <button class="border border-gray-300 rounded p-1.5 bg-gray-100"
                on:click=move |_| set_snapshots(vec![])>
                Clear
            </button>
        </p>
        <DisplaySnapshots snapshots=snapshots />
        </main>
    }
}
