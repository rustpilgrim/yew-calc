use gloo_net::http::Request;
use serde::{Serialize, Deserialize};
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video
}

#[function_component(App)]
fn app() -> Html {
    //let videos = use_state(|| vec![]);

    let temp_value: UseStateHandle<Option<f32>> = use_state(|| None);
    let calc_memory: UseStateHandle<f32> = use_state(|| 0.0);
    let calc_action: UseStateHandle<Option<CalcOperation>> = use_state(|| None);

    let calcmemory_onclick = {
        let calc_memory = calc_memory.clone();
        Callback::from(move |_| calc_memory.set(*calc_memory + 1.0))
    };
/*
    {
        let videos = videos.clone();
        use_effect_with((), move |_| {
            let videos = videos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                videos.set(fetched_videos);
            });
            || ()
        });
    }
    
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| {
            selected_video.set(Some(video))
        })
    };
    
    let details = selected_video.as_ref().map(|video| html! {
        <VideoDetails video={video.clone()} />
    });
*/
    html! {
        <>
            //<div>
            //    <h3>{"Videos to watch"}</h3>
            //    <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            //</div>
            //{ for details }
            <Calculator operation={CalcOperation::ADD} memory={*calc_memory} temp={None} />
            <div>
                <button class="button-1" onclick={calcmemory_onclick}>{ "+1" }</button>
            <p>
                <b>{format!("Current value: {}", *calc_memory)}</b>
            </p>
        </div>
        </>
    }
}

#[function_component(VideosList)]
fn video_list(VideosListProps {videos, on_click}: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
        let on_video_select = {
            let on_click = on_click.clone();
            let video = video.clone();
            Callback::from(move |_| {
                on_click.emit(video.clone())
            })
        };

        html! {
            <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
        }
    }).collect()
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum CalcOperation {
    ADD,
    SUB,
    MUL,
    DIV
}

impl CalcOperation {
    pub fn to_string(&self) -> String {
        match self {
            CalcOperation::ADD => "ADD".to_string(),
            CalcOperation::SUB => "SUB".to_string(),
            CalcOperation::MUL => "MUL".to_string(),
            CalcOperation::DIV => "DIV".to_string()
        }
    }
}

#[derive(Properties, PartialEq, Clone, Serialize, Deserialize)]
struct CalculatorProps {
    operation: CalcOperation,
    memory: f32,
    temp: Option<f32>
}

#[function_component(Calculator)]
fn calc_component(CalculatorProps { operation, memory, temp }: &CalculatorProps) -> Html {
    html! {
        <div>
            <h1>{ "calc placeholder" }</h1>
                <div>
                    <div>
                    {match *temp {
                        Some(v) => v,
                        None => *memory,
                    }}
                    </div>
                </div>
            <h3>{operation.to_string()}</h3>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
