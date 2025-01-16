use serde::{Deserialize, Serialize};
use std::io::{stdin, BufRead, Read};
use std::sync::{Arc, Mutex};
use webrtc::api::interceptor_registry::register_default_interceptors;
use webrtc::api::media_engine::MediaEngine;
use webrtc::api::APIBuilder;
use webrtc::data_channel::RTCDataChannel;
use webrtc::interceptor::registry::Registry;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::rtp_transceiver::rtp_codec::RTPCodecType;

#[derive(Serialize, Deserialize)]
pub struct Rtc {
    pub channel: String,
    pub sdp: String,
    pub is_video: bool,
    pub is_audio: bool,
}

impl Rtc {
    pub async fn connect(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // 初始化 MediaEngine 和 API
        let mut media_engine = MediaEngine::default();
        register_default_interceptors(Registry::new(), &mut media_engine)?;

        let api = APIBuilder::new().with_media_engine(media_engine).build();

        // 创建 PeerConnection
        let peer_connection = Arc::new(api.new_peer_connection(Default::default()).await?);

        // 创建数据通道
        let data_channel = peer_connection
            .create_data_channel(&self.channel, None)
            .await?;
        // 创建 offer
        let offer = peer_connection.create_offer(None).await?;
        peer_connection.set_local_description(offer.clone()).await?;

        println!("Offer SDP:\n{}", offer.sdp);

        let answer = RTCSessionDescription::answer(self.sdp.clone())?;
        peer_connection.set_remote_description(answer).await?;

        if self.is_audio {
            peer_connection
                .add_transceiver_from_kind(RTPCodecType::Audio, None)
                .await?;
        }
        if self.is_video {
            peer_connection
                .add_transceiver_from_kind(RTPCodecType::Video, None)
                .await?;
        }
        self.setup_data_channel(data_channel).await;
        Ok(())
    }

    async fn setup_data_channel(&self, data_channel: Arc<RTCDataChannel>) {
        let data_channel_1 = data_channel.clone();
        let data_channel_2 = data_channel.clone();
        data_channel_1.on_open(Box::new(move || {
            Box::pin(async move {
                println!("Data channel opened!");
                data_channel.send_text("Hello WebRTC!").await.unwrap();
            })
        }));

        data_channel_2.on_message(Box::new(move |msg| {
            Box::pin(async move {
                println!("Received message: {:?}", msg.data.slice(0..));
            })
        }));
    }
}
