#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/link_rs.rs"));
}

use sys::*;
use std::os::raw::c_void;

/// # Represents a participant in a Link session.
/// 
/// Each Link instance has its own session state which
/// represents a beat timeline and a transport start/stop state. The
/// timeline starts running from beat 0 at the initial tempo when
/// constructed. The timeline always advances at a speed defined by
/// its current tempo, even if transport is stopped. Synchronizing to the
/// transport start/stop state of Link is optional for every peer.
/// The transport start/stop state is only shared with other peers when
/// start/stop synchronization is enabled.
/// 
/// A Link instance is initially disabled after construction, which
/// means that it will not communicate on the network. Once enabled,
/// a Link instance initiates network communication in an effort to
/// discover other peers. When peers are discovered, they immediately
/// become part of a shared Link session.
/// 
/// Each method of the Link type documents its thread-safety and
/// realtime-safety properties. When a method is marked thread-safe,
/// it means it is safe to call from multiple threads
/// concurrently. When a method is marked realtime-safe, it means that
/// it does not block and is appropriate for use in the thread that
/// performs audio IO.
/// 
/// Link provides one session state capture/commit method pair for use
/// in the audio thread and one for all other application contexts. In
/// general, modifying the session state should be done in the audio
/// thread for the most accurate timing results. The ability to modify
/// the session state from application threads should only be used in
/// cases where an application's audio thread is not actively running
/// or if it doesn't generate audio at all. Modifying the Link session
/// state from both the audio thread and an application thread
/// concurrently is not advised and will potentially lead to unexpected
/// behavior.
pub struct Link {
    wlink: *mut WLink,
}

impl Drop for Link {
    fn drop(&mut self) {
        unsafe { Link_destroy(self.wlink) }
        println!("Link destroyed!")
    }
}

impl Link {
    /// Construct with an initial tempo.
    pub fn new(bpm: f64) -> Link {
        Link { wlink: unsafe { Link_create(bpm) } }
    }

    /// Is Link currently enabled?
    /// * Thread-safe: yes
    /// * Realtime-safe: yes
    pub fn is_enabled(&self) -> bool {
        unsafe { Link_isEnabled(self.wlink) }
    }

    /// Enable/disable Link.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    pub fn enable(&mut self, enable: bool) {
        unsafe { Link_enable(self.wlink, enable) }
    }

    /// Is start/stop synchronization enabled?
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    pub fn is_start_stop_sync_enabled(&self) -> bool {
        unsafe { Link_isStartStopSyncEnabled(self.wlink) }
    }

    /// Enable start/stop synchronization.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    pub fn enable_start_stop_sync(&mut self, enable: bool) {
        unsafe { Link_enableStartStopSync(self.wlink, enable) }
    }

    /// How many peers are currently connected in a Link session?
    /// * Thread-safe: yes
    /// * Realtime-safe: yes
    pub fn num_peers(&self) -> usize {
        unsafe { Link_numPeers(self.wlink) }
    }

    /// Register a callback to be notified when the number of
    /// peers in the Link session changes.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    /// 
    /// The callback is invoked on a Link-managed thread.
    pub fn set_num_peers_callback(&mut self, callback: extern fn(usize)) {
        unsafe {
            let cb = callback as unsafe extern fn(usize);
            Link_setNumPeersCallback(self.wlink, Some(cb));
        }
    }

    /// Register a callback to be notified when the session tempo changes.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    /// 
    /// The callback is invoked on a Link-managed thread.
    pub fn set_tempo_callback(&mut self, callback: extern fn(f64)) {
        unsafe {
            let cb = callback as unsafe extern fn(f64);
            Link_setTempoCallback(self.wlink, Some(cb));
        }
    }

    /// Register a callback to be notified when the state of
    /// start/stop isPlaying changes.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    /// 
    /// The callback is invoked on a Link-managed thread.
    pub fn set_start_stop_callback(&mut self, callback: extern fn(bool)) {
        unsafe {
            let cb = callback as unsafe extern fn(bool);
            Link_setStartStopCallback(self.wlink, Some(cb));
        }
    }

    /// The clock used by Link.
    /// * Thread-safe: yes
    /// * Realtime-safe: yes
    /// 
    /// The Clock type is a platform-dependent
    /// representation of the system clock. It exposes a `ticks()` method
    /// that returns the current ticks of the system clock as well as
    /// `micros()`, which is a normalized representation of the current system
    /// time in std::chrono::microseconds. It also provides conversion
    /// functions `ticksToMicros()` and `microsToTicks()` to faciliate
    /// converting between these units.
    pub fn clock(&self) -> Clock {
        Clock { wc: unsafe { Link_clock(self.wlink) } }
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Capture the current Link Session State from the audio thread.
    /// * Thread-safe: no
    /// * Realtime-safe: yes
    /// 
    /// This method should ONLY be called in the audio thread
    /// and must not be accessed from any other threads. The returned
    /// object stores a snapshot of the current Link Session State, so it
    /// should be captured and used in a local scope. Storing the
    /// Session State for later use in a different context is not advised
    /// because it will provide an outdated view.
    pub fn capture_audio_session_state(&self) -> SessionState {
        unimplemented!()
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Commit the given Session State to the Link session from the audio thread.
    /// * Thread-safe: no
    /// * Realtime-safe: yes
    /// 
    /// This method should ONLY be called in the audio
    /// thread. The given Session State will replace the current Link
    /// state. Modifications will be communicated to other peers in the
    /// session.
    pub fn commit_audio_session_state(&mut self, _ss: SessionState) {
        unimplemented!()
    }

    /// Capture the current Link Session State from an application thread.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    /// 
    /// Provides a mechanism for capturing the Link Session
    /// State from an application thread (other than the audio thread).
    /// The returned Session State stores a snapshot of the current Link
    /// state, so it should be captured and used in a local scope.
    /// Storing the it for later use in a different context is not
    /// advised because it will provide an outdated view.
    pub fn capture_app_session_state(&self) -> SessionState {
        let wss = unsafe { Link_captureAppSessionState(self.wlink) };
        SessionState { wss }
    }

    pub fn with_app_session_state<F>(&self, f: F)
        where F: FnMut(SessionState)
    {
        let user_data = &f as *const _ as *mut c_void;
        unsafe {
            Link_withAppSessionState(self.wlink, Some(closure_wrapper::<F>), user_data);
        }

        extern fn closure_wrapper<F>(closure: *mut c_void, wss: *mut WSessionState)
            where F: FnMut(SessionState)
        {
            let opt_closure = closure as *mut Option<F>;
            unsafe {
                let mut fnx = (*opt_closure).take().unwrap();
                let ss = SessionState { wss };
                fnx(ss);
            }
        }
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Commit the given Session State to the Link session from an
    /// application thread.
    /// * Thread-safe: yes
    /// * Realtime-safe: no
    /// 
    /// The given Session State will replace the current Link
    /// Session State. Modifications of the Session State will be
    /// communicated to other peers in the session.
    pub fn commit_app_session_state(&mut self, _ss: SessionState) {
        unimplemented!()
    }
}

/// # Representation of a timeline and the start/stop state
/// 
/// A SessionState object is intended for use in a local scope within
/// a single thread - none of its methods are thread-safe. All of its methods are
/// non-blocking, so it is safe to use from a realtime thread.
/// It provides functions to observe and manipulate the timeline and start/stop
/// state.
/// 
/// The timeline is a representation of a mapping between time and beats for varying
/// quanta.
/// 
/// The start/stop state represents the user intention to start or stop transport at
/// a specific time. Start stop synchronization is an optional feature that allows to
/// share the user request to start or stop transport between a subgroup of peers in
/// a Link session. When observing a change of start/stop state, audio playback of a
/// peer should be started or stopped the same way it would have happened if the user
/// had requested that change at the according time locally. The start/stop state can
/// only be changed by the user. This means that the current local start/stop state
/// persists when joining or leaving a Link session. After joining a Link session
/// start/stop change requests will be communicated to all connected peers.
pub struct SessionState {
    wss: *mut WSessionState,
}

// impl Drop for SessionState {
//     fn drop(&mut self) {
//         unsafe { SessionState_destroy(self.wss) }
//     }
// }

impl SessionState {
    /// The tempo of the timeline, in bpm.
    pub fn tempo(&self) -> f64 {
        unsafe { SessionState_tempo(self.wss) }
    }

    /// Set the timeline tempo to the given bpm value, taking
    /// effect at the given time.
    pub fn set_tempo(&mut self, bpm: f64, at_time: i64) {
        unsafe { SessionState_setTempo(self.wss, bpm, at_time) }
    }

    /// Get the beat value corresponding to the given time
    /// for the given quantum.
    ///
    /// The magnitude of the resulting beat value is
    /// unique to this Link instance, but its phase with respect to
    /// the provided quantum is shared among all session
    /// peers. For non-negative beat values, the following
    /// property holds: fmod(beatAtTime(t, q), q) == phaseAtTime(t, q)
    pub fn beat_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { SessionState_beatAtTime(self.wss, time, quantum) }
    }

    /// Get the session phase at the given time for the given quantum.
    /// 
    /// The result is in the interval [0, quantum). The
    /// result is equivalent to fmod(beatAtTime(t, q), q) for
    /// non-negative beat values. This method is convenient if the
    /// client is only interested in the phase and not the beat
    /// magnitude. Also, unlike fmod, it handles negative beat values
    /// correctly.
    pub fn phase_at_time(&self, time: i64, quantum: f64) -> f64 {
        unsafe { SessionState_phaseAtTime(self.wss, time, quantum) }
    }

    /// Get the time at which the given beat occurs for the
    /// given quantum.
    /// 
    /// The inverse of beatAtTime, assuming a constant
    /// tempo. beatAtTime(timeAtBeat(b, q), q) === b.
    pub fn time_at_beat(&self, beat: f64, quantum: f64) -> i64 {
        unsafe { SessionState_timeAtBeat(self.wss, beat, quantum) }
    }

    /// Attempt to map the given beat to the given time in the
    /// context of the given quantum.
    /// 
    /// This method behaves differently depending on the
    /// state of the session. If no other peers are connected,
    /// then this instance is in a session by itself and is free to
    /// re-map the beat/time relationship whenever it pleases. In this
    /// case, beatAtTime(time, quantum) == beat after this method has
    /// been called.
    /// 
    /// If there are other peers in the session, this instance
    /// should not abruptly re-map the beat/time relationship in the
    /// session because that would lead to beat discontinuities among
    /// the other peers. In this case, the given beat will be mapped
    /// to the next time value greater than the given time with the
    /// same phase as the given beat.
    /// 
    /// This method is specifically designed to enable the concept of
    /// "quantized launch" in client applications. If there are no other
    /// peers in the session, then an event (such as starting
    /// transport) happens immediately when it is requested. If there
    /// are other peers, however, we wait until the next time at which
    /// the session phase matches the phase of the event, thereby
    /// executing the event in-phase with the other peers in the
    /// session. The client only needs to invoke this method to
    /// achieve this behavior and should not need to explicitly check
    /// the number of peers.
    pub fn request_beat_at_time(&mut self, beat: f64, time: i64, quantum: f64) {
        unsafe { SessionState_requestBeatAtTime(self.wss, beat, time, quantum) }
    }

    /// Rudely re-map the beat/time relationship for all peers
    /// in a session.
    /// 
    /// DANGER: This method should only be needed in
    /// certain special circumstances. Most applications should not
    /// use it. It is very similar to requestBeatAtTime except that it
    /// does not fall back to the quantizing behavior when it is in a
    /// session with other peers. Calling this method will
    /// unconditionally map the given beat to the given time and
    /// broadcast the result to the session. This is very anti-social
    /// behavior and should be avoided.
    /// 
    /// One of the few legitimate uses of this method is to
    /// synchronize a Link session with an external clock source. By
    /// periodically forcing the beat/time mapping according to an
    /// external clock source, a peer can effectively bridge that
    /// clock into a Link session. Much care must be taken at the
    /// application layer when implementing such a feature so that
    /// users do not accidentally disrupt Link sessions that they may
    /// join.
    pub fn force_beat_at_time(&mut self, beat: f64, time: i64, quantum: f64) {
        unsafe { SessionState_forceBeatAtTime(self.wss, beat, time, quantum) }
    }

    /// Set if transport should be playing or stopped, taking effect
    /// at the given time.
    pub fn set_is_playing(&mut self, is_playing: bool, time: i64) {
        unsafe { SessionState_setIsPlaying(self.wss, is_playing, time) }
    }

    /// Is transport playing?
    pub fn is_playing(&self) -> bool {
        unsafe { SessionState_isPlaying(self.wss) }
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Get the time at which a transport start/stop occurs.
    pub fn time_for_is_playing(&self) -> i64 {
        unimplemented!()
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Convenience function to attempt to map the given beat to the time
    /// when transport is starting to play in context of the given quantum.
    /// This function evaluates to a no-op if isPlaying() equals false.
    pub fn request_beat_at_start_playing_time(&mut self, _beat: f64, _quantum: f64) {
        unimplemented!()
    }

    /// ![](https://upload.wikimedia.org/wikipedia/commons/5/59/QSicon_missing.svg)
    /// Convenience function to start or stop transport at a given time and
    /// attempt to map the given beat to this time in context of the given quantum.
    pub fn set_is_playing_and_request_beat_at_time(&mut self,
        _is_playing: bool, _time: i64, _beat: f64, _quantum: f64) {

        unimplemented!()
    }
}

pub struct Clock {
    wc: *mut WClock,
}

impl Drop for Clock{
    fn drop(&mut self) {
        unsafe { Clock_destroy(self.wc) }
    }
}

impl Clock {
    pub fn ticks_to_micros(&self, ticks: u64) -> i64 {
        unsafe { Clock_ticksToMicros(self.wc, ticks) }
    }

    pub fn micros_to_ticks(&self, micros: i64) -> u64 {
        unsafe { Clock_microsToTicks(self.wc, micros) }
    }

    pub fn ticks(&self) -> u64 {
        unsafe { Clock_ticks(self.wc) }
    }

    pub fn micros(&self) -> i64 {
        unsafe { Clock_micros(self.wc) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
