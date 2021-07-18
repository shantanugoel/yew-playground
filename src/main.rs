use std::{fmt, str::FromStr};

use chrono::{Duration, NaiveDate, TimeZone};
use chrono_tz::{America::Los_Angeles, Asia::Kolkata};
use csv::ReaderBuilder;
use serde::Deserialize;
use simsearch::SimSearch;
use yew::{prelude::*, services::ConsoleService};

enum Msg {
    AddOne,
    DatePick(ChangeData),
    StringData(String),
    CityInput(InputData),
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
struct TzData {
    city: String,
    country: String,
    timezone: String,
}

impl fmt::Display for TzData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.city, self.country, self.timezone)
    }
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
    ref_date: NaiveDate,
    tz_data: Vec<TzData>,
    search_data: SimSearch<String>,
    found_data: Vec<String>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // let data = String::from("City, Country, Timezone");
        // let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());
        let data = include_str!("cities.csv");
        // let raw_data: Vec<String> = data.lines().into_iter().map(|s| s.to_string()).collect();
        let mut search_data = SimSearch::new();
        for line in data.lines() {
            search_data.insert(line.to_string(), line);
        }
        let mut rdr = ReaderBuilder::new().from_reader(data.as_bytes());
        let mut model = Self {
            link,
            value: 0,
            ref_date: NaiveDate::from_ymd(2021, 07, 18),
            tz_data: Vec::new(),
            search_data: search_data,
            found_data: Vec::new(),
        };

        for result in rdr.deserialize() {
            let record: TzData = result.unwrap();
            model.tz_data.push(record.clone());
            // ConsoleService::log(record.to_string().as_ref());
        }
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DatePick(x) => match x {
                ChangeData::Files(_) => ConsoleService::log("file"),
                ChangeData::Select(_) => ConsoleService::log("selec"),
                ChangeData::Value(v) => {
                    ConsoleService::log(v.as_ref());
                    self.ref_date = NaiveDate::from_str(v.as_str()).unwrap();
                }
            },
            Msg::StringData(x) => ConsoleService::log(x.as_ref()),
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
            }
            Msg::CityInput(x) => {
                ConsoleService::log(x.value.as_ref());
                self.found_data.clear();
                let results: Vec<String> = self
                    .search_data
                    .search(x.value.as_ref())
                    .iter()
                    .take(5)
                    .map(|s| s.clone())
                    .collect();
                self.found_data.extend(results);
                ConsoleService::log(self.found_data.len().to_string().as_ref());
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        ConsoleService::log("Change");
        false
    }

    fn view(&self) -> Html {
        // let blr_time = Local::today().with_timezone(&Kolkata).and_hms(9, 0, 0);
        // let blr_time = Kolkata.ymd(2021, 7, 18).and_hms(9, 0, 0);
        let blr_time = Kolkata
            .from_local_date(&self.ref_date)
            .unwrap()
            .and_hms(9, 0, 0);
        let mtv_time = blr_time.with_timezone(&Los_Angeles);
        let range = 0..24;
        let cities = vec!["BLR", "MTV"];
        html! {
            <div>
                // <button onclick=self.link.callback(|x| x)>{ "+1" }</button>
                <p>{ self.value } { self.tz_data.len()}</p>
                <p>{ " Hello"}</p>
                <p>{"Blr: "}{blr_time} {" MTV: "}{mtv_time}</p>
                <input type="date" onchange=self.link.callback(|x| Msg::DatePick(x))/>
                <table>
                <thead>
                <tr>
                {
                for cities.iter().map(|city| {html! {<th> {city} </th>} })
                }
                </tr>
                </thead>
                {
                    for range.map(|x| {html! {<tr>
                        <td> { {(blr_time + Duration::hours(x)).format("%l:%M %p %e %b")} } </td>
                        <td> { {(mtv_time + Duration::hours(x)).format("%l:%M %p %e %b")} } </td>
                        </tr>}})
                }
                </table>
                <input oninput=self.link.callback(|x| Msg::CityInput(x))/>
                {
                    for self.found_data.clone().into_iter().map(|s| {html!{
                        <p> {s } </p>
                    }})
                }
            </div>
        }
    }
}

fn main() {
    // let rdr = ReaderBuilder::new().from_path("cities.csv");
    yew::start_app::<Model>();
}
