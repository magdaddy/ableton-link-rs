#include <unistd.h>
#include <ableton/Link.hpp>

#include "link_rs.h"


// Link

WLink* Link_create(double bpm) {
    return reinterpret_cast<WLink*>(new ableton::Link(bpm));
}

void Link_destroy(WLink* lp) { delete reinterpret_cast<ableton::Link*>(lp); }

bool Link_isEnabled(WLink* lp) {
    return reinterpret_cast<ableton::Link*>(lp)->isEnabled();
}

void Link_enable(WLink* lp, bool enable) {
    reinterpret_cast<ableton::Link*>(lp)->enable(enable);
}

bool Link_isStartStopSyncEnabled(WLink* lp) {
    return reinterpret_cast<ableton::Link*>(lp)->isStartStopSyncEnabled();
}

void Link_enableStartStopSync(WLink* lp, bool enable) {
    reinterpret_cast<ableton::Link*>(lp)->enableStartStopSync(enable);
}

size_t Link_numPeers(WLink* lp) {
    return reinterpret_cast<ableton::Link*>(lp)->numPeers();
}

void Link_setNumPeersCallback(WLink* lp, void (*callback)(size_t)) {
    reinterpret_cast<ableton::Link*>(lp)->setNumPeersCallback(callback);
}

void Link_setTempoCallback(WLink* lp, void (*callback)(double)) {
    reinterpret_cast<ableton::Link*>(lp)->setTempoCallback(callback);
}

void Link_setStartStopCallback(WLink* lp, void (*callback)(bool)) {
    reinterpret_cast<ableton::Link*>(lp)->setStartStopCallback(callback);
}

WClock* Link_clock(WLink* lp) {
    auto c = reinterpret_cast<ableton::Link*>(lp)->clock();
    auto cp = new ableton::Link::Clock(c);
    return reinterpret_cast<WClock*>(cp);
}

WSessionState* Link_captureAppSessionState(WLink* lp) {
    auto sss = reinterpret_cast<ableton::Link*>(lp)->captureAppSessionState();
    auto ssh = new ableton::Link::SessionState(sss);
    return reinterpret_cast<WSessionState*>(ssh);
}

void Link_withAppSessionState(WLink* lp, RustClosurePtr cp, void* closure_data) {
    auto ss = reinterpret_cast<ableton::Link*>(lp)->captureAppSessionState();
    cp(closure_data, reinterpret_cast<WSessionState*>(&ss));
}

// SessionState

void SessionState_destroy(WSessionState* ssp) { 
    delete reinterpret_cast<ableton::Link::SessionState*>(ssp);
}

double SessionState_tempo(WSessionState* ssp) {
    return reinterpret_cast<ableton::Link::SessionState*>(ssp)->tempo();
}

void SessionState_setTempo(WSessionState* ssp, double bpm, int64_t atTime) {
    std::chrono::microseconds t(atTime);
    reinterpret_cast<ableton::Link::SessionState*>(ssp)->setTempo(bpm, t);
}

double SessionState_beatAtTime(WSessionState* ssp, int64_t time, double quantum) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    std::chrono::microseconds t(time);
    return asp->beatAtTime(t, quantum);
}

double SessionState_phaseAtTime(WSessionState* ssp, int64_t time, double quantum) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    std::chrono::microseconds t(time);
    return asp->phaseAtTime(t, quantum);
}

int64_t SessionState_timeAtBeat(WSessionState* ssp, double beat, double quantum) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    auto micros = asp->timeAtBeat(beat, quantum);
    return micros.count();
}

void SessionState_requestBeatAtTime(WSessionState* ssp, double beat, int64_t time, double quantum) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    std::chrono::microseconds t(time);
    asp->requestBeatAtTime(beat, t, quantum);
}

void SessionState_forceBeatAtTime(WSessionState* ssp, double beat, int64_t time, double quantum) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    std::chrono::microseconds t(time);
    asp->forceBeatAtTime(beat, t, quantum);
}

void SessionState_setIsPlaying(WSessionState* ssp, bool isPlaying, int64_t time) {
    auto asp = reinterpret_cast<ableton::Link::SessionState*>(ssp);
    std::chrono::microseconds t(time);
    asp->setIsPlaying(isPlaying, t);
}

bool SessionState_isPlaying(WSessionState* ssp) {
    return reinterpret_cast<ableton::Link::SessionState*>(ssp)->isPlaying();
}

// Clock

void Clock_destroy(WClock* cp) {
    delete reinterpret_cast<ableton::Link::Clock*>(cp);
}

int64_t Clock_ticksToMicros(WClock* cp, uint64_t ticks) {
    return reinterpret_cast<ableton::Link::Clock*>(cp)->ticksToMicros(ticks).count();
}

uint64_t Clock_microsToTicks(WClock* cp, int64_t micros) {
    std::chrono::microseconds t(micros);
    return reinterpret_cast<ableton::Link::Clock*>(cp)->microsToTicks(t);
}

uint64_t Clock_ticks(WClock* cp) {
    return reinterpret_cast<ableton::Link::Clock*>(cp)->ticks();
}

int64_t Clock_micros(WClock* cp) {
    return reinterpret_cast<ableton::Link::Clock*>(cp)->micros().count();
}

