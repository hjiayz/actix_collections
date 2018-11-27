extern crate actix;
extern crate multimap;
use actix::prelude::*;
use multimap::MultiMap;
use std::cmp::Eq;
use std::cmp::Ord;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;
use std::marker::PhantomData;

#[derive(Default)]
pub struct ActixCollections<T>(T);

impl<T: 'static> Actor for ActixCollections<T> {
    type Context = Context<Self>;
}

impl<T: 'static> actix::Supervised for ActixCollections<T> {}

impl<T: 'static + Default> actix::registry::SystemService for ActixCollections<T> {}

impl<T: 'static + Default> actix::registry::ArbiterService for ActixCollections<T> {}

pub struct Push<T>(T);

impl<T: 'static> actix::Message for Push<T> {
    type Result = ();
}

impl<T: 'static> Handler<Push<T>> for ActixCollections<Vec<T>> {
    type Result = ();

    fn handle(&mut self, msg: Push<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push(msg.0)
    }
}

impl<T: 'static + Ord> Handler<Push<T>> for ActixCollections<BinaryHeap<T>> {
    type Result = ();

    fn handle(&mut self, msg: Push<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push(msg.0)
    }
}

pub struct Pop<T>(PhantomData<T>);

impl<T: 'static> actix::Message for Pop<T> {
    type Result = Option<T>;
}

impl<T: 'static> Handler<Pop<T>> for ActixCollections<Vec<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: Pop<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop()
    }
}

impl<T: 'static + Ord> Handler<Pop<T>> for ActixCollections<BinaryHeap<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: Pop<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop()
    }
}

pub struct PushFront<T>(T);

impl<T: 'static> actix::Message for PushFront<T> {
    type Result = ();
}

impl<T: 'static> Handler<PushFront<T>> for ActixCollections<VecDeque<T>> {
    type Result = ();

    fn handle(&mut self, msg: PushFront<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push_front(msg.0)
    }
}

impl<T: 'static> Handler<PushFront<T>> for ActixCollections<LinkedList<T>> {
    type Result = ();

    fn handle(&mut self, msg: PushFront<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push_front(msg.0)
    }
}

pub struct PushBack<T>(T);

impl<T: 'static> actix::Message for PushBack<T> {
    type Result = ();
}

impl<T: 'static> Handler<PushBack<T>> for ActixCollections<VecDeque<T>> {
    type Result = ();

    fn handle(&mut self, msg: PushBack<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push_back(msg.0)
    }
}

impl<T: 'static> Handler<PushBack<T>> for ActixCollections<LinkedList<T>> {
    type Result = ();

    fn handle(&mut self, msg: PushBack<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.push_back(msg.0)
    }
}

pub struct PopFront<T>(PhantomData<T>);

impl<T: 'static> actix::Message for PopFront<T> {
    type Result = Option<T>;
}

impl<T: 'static> Handler<PopFront<T>> for ActixCollections<VecDeque<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: PopFront<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop_front()
    }
}

impl<T: 'static> Handler<PopFront<T>> for ActixCollections<LinkedList<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: PopFront<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop_front()
    }
}

pub struct PopBack<T>(PhantomData<T>);

impl<T: 'static> actix::Message for PopBack<T> {
    type Result = Option<T>;
}

impl<T: 'static> Handler<PopBack<T>> for ActixCollections<VecDeque<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: PopBack<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop_back()
    }
}

impl<T: 'static> Handler<PopBack<T>> for ActixCollections<LinkedList<T>> {
    type Result = Option<T>;

    fn handle(&mut self, _msg: PopBack<T>, _: &mut Context<Self>) -> Self::Result {
        self.0.pop_back()
    }
}

pub struct Len;

impl actix::Message for Len {
    type Result = usize;
}

impl<T: 'static> Handler<Len> for ActixCollections<Vec<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<T: 'static> Handler<Len> for ActixCollections<VecDeque<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<T: 'static + Eq + Hash> Handler<Len> for ActixCollections<HashSet<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<K: 'static + Eq + Hash, T: 'static> Handler<Len> for ActixCollections<HashMap<K, T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<T: 'static> Handler<Len> for ActixCollections<BTreeSet<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<K: 'static, T: 'static> Handler<Len> for ActixCollections<BTreeMap<K, T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<T: 'static> Handler<Len> for ActixCollections<LinkedList<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<T: 'static + Ord> Handler<Len> for ActixCollections<BinaryHeap<T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

impl<K: 'static + Eq + Hash, T: 'static> Handler<Len> for ActixCollections<MultiMap<K, T>> {
    type Result = usize;

    fn handle(&mut self, _msg: Len, _: &mut Context<Self>) -> Self::Result {
        self.0.len()
    }
}

pub struct Contains<V>(V);

impl<V> actix::Message for Contains<V> {
    type Result = bool;
}

impl<V: 'static + Eq> Handler<Contains<V>> for ActixCollections<Vec<V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<V>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains(&msg.0)
    }
}

impl<V: 'static + Eq> Handler<Contains<V>> for ActixCollections<VecDeque<V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<V>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains(&msg.0)
    }
}

impl<V: 'static + Eq> Handler<Contains<V>> for ActixCollections<LinkedList<V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<V>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains(&msg.0)
    }
}

impl<K: 'static + Hash + Eq, V: 'static> Handler<Contains<K>> for ActixCollections<HashMap<K, V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<K>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains_key(&msg.0)
    }
}

impl<V: 'static + Hash + Eq> Handler<Contains<V>> for ActixCollections<HashSet<V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<V>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains(&msg.0)
    }
}

impl<K: 'static + Ord + Eq, V: 'static> Handler<Contains<K>> for ActixCollections<BTreeMap<K, V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<K>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains_key(&msg.0)
    }
}

impl<V: 'static + Ord + Eq> Handler<Contains<V>> for ActixCollections<BTreeSet<V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<V>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains(&msg.0)
    }
}

impl<K: 'static + Hash + Eq, V: 'static> Handler<Contains<K>> for ActixCollections<MultiMap<K, V>> {
    type Result = bool;

    fn handle(&mut self, msg: Contains<K>, _: &mut Context<Self>) -> Self::Result {
        self.0.contains_key(&msg.0)
    }
}

pub struct Insert<K, V> {
    key: K,
    value: V,
}

impl<K, V: 'static> actix::Message for Insert<K, V> {
    type Result = ();
}

impl<V> Handler<Insert<usize, V>> for ActixCollections<Vec<V>>
where
    V: Clone + Default + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: Insert<usize, V>, _: &mut Context<Self>) -> Self::Result {
        if msg.key > self.0.len() {
            self.0.resize(msg.key, Default::default())
        }
        self.0.insert(msg.key, msg.value)
    }
}

impl<V> Handler<Insert<usize, V>> for ActixCollections<VecDeque<V>>
where
    V: Clone + Default + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: Insert<usize, V>, _: &mut Context<Self>) -> Self::Result {
        if msg.key > self.0.len() {
            self.0.resize(msg.key, Default::default())
        }
        self.0.insert(msg.key, msg.value)
    }
}

impl<K, V> Handler<Insert<K, V>> for ActixCollections<HashMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: Insert<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.insert(msg.key, msg.value);
    }
}

impl<V> Handler<Insert<(), V>> for ActixCollections<HashSet<V>>
where
    V: Clone + Default + Eq + Hash + 'static,
{
    type Result = ();
    fn handle(&mut self, msg: Insert<(), V>, _: &mut Context<Self>) -> Self::Result {
        self.0.insert(msg.value);
    }
}

impl<K, V> Handler<Insert<K, V>> for ActixCollections<BTreeMap<K, V>>
where
    K: 'static + Ord,
    V: Clone + Default + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: Insert<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.insert(msg.key, msg.value);
    }
}

impl<V> Handler<Insert<(), V>> for ActixCollections<BTreeSet<V>>
where
    V: Clone + Default + Ord + 'static,
{
    type Result = ();
    fn handle(&mut self, msg: Insert<(), V>, _: &mut Context<Self>) -> Self::Result {
        self.0.insert(msg.value);
    }
}

impl<K, V> Handler<Insert<K, V>> for ActixCollections<MultiMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = ();

    fn handle(&mut self, msg: Insert<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.insert(msg.key, msg.value);
    }
}

pub struct Remove<K, V> {
    key: K,
    value: PhantomData<V>,
}

impl<K, V: 'static> actix::Message for Remove<K, V> {
    type Result = Option<V>;
}

impl<V> Handler<Remove<usize, V>> for ActixCollections<Vec<V>>
where
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Remove<usize, V>, _: &mut Context<Self>) -> Self::Result {
        if msg.key >= self.0.len() {
            return None;
        }
        Some(self.0.remove(msg.key))
    }
}

impl<V> Handler<Remove<usize, V>> for ActixCollections<VecDeque<V>>
where
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Remove<usize, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.remove(msg.key)
    }
}

impl<K, V> Handler<Remove<K, V>> for ActixCollections<HashMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Remove<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.remove(&msg.key)
    }
}

impl<V> Handler<Remove<V, V>> for ActixCollections<HashSet<V>>
where
    V: Clone + Default + Eq + Hash + 'static,
{
    type Result = Option<V>;
    fn handle(&mut self, msg: Remove<V, V>, _: &mut Context<Self>) -> Self::Result {
        if self.0.remove(&msg.key) {
            Some(msg.key)
        } else {
            None
        }
    }
}

impl<K, V> Handler<Remove<K, V>> for ActixCollections<BTreeMap<K, V>>
where
    K: 'static + Ord,
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Remove<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.remove(&msg.key)
    }
}

impl<V> Handler<Remove<V, V>> for ActixCollections<BTreeSet<V>>
where
    V: Clone + Default + Ord + 'static,
{
    type Result = Option<V>;
    fn handle(&mut self, msg: Remove<V, V>, _: &mut Context<Self>) -> Self::Result {
        if self.0.remove(&msg.key) {
            Some(msg.key)
        } else {
            None
        }
    }
}

pub struct RemoveVec<K, V> {
    key: K,
    value: PhantomData<V>,
}

impl<K, V: 'static> actix::Message for RemoveVec<K, V> {
    type Result = Option<Vec<V>>;
}

impl<K, V> Handler<RemoveVec<K, V>> for ActixCollections<MultiMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = Option<Vec<V>>;

    fn handle(&mut self, msg: RemoveVec<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.remove(&msg.key)
    }
}

pub struct Get<K, V> {
    key: K,
    value: PhantomData<V>,
}

impl<K, V: 'static> actix::Message for Get<K, V> {
    type Result = Option<V>;
}

impl<V> Handler<Get<usize, V>> for ActixCollections<Vec<V>>
where
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Get<usize, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.get(msg.key).cloned()
    }
}

impl<V> Handler<Get<usize, V>> for ActixCollections<VecDeque<V>>
where
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Get<usize, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.get(msg.key).cloned()
    }
}

impl<K, V> Handler<Get<K, V>> for ActixCollections<HashMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Get<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.get(&msg.key).cloned()
    }
}

impl<K, V> Handler<Get<K, V>> for ActixCollections<BTreeMap<K, V>>
where
    K: 'static + Ord,
    V: Clone + Default + 'static,
{
    type Result = Option<V>;

    fn handle(&mut self, msg: Get<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.get(&msg.key).cloned()
    }
}

pub struct GetVec<K, V> {
    key: K,
    value: PhantomData<V>,
}

impl<K, V: 'static> actix::Message for GetVec<K, V> {
    type Result = Option<Vec<V>>;
}

impl<K, V> Handler<GetVec<K, V>> for ActixCollections<MultiMap<K, V>>
where
    K: 'static + Eq + Hash,
    V: Clone + Default + 'static,
{
    type Result = Option<Vec<V>>;

    fn handle(&mut self, msg: GetVec<K, V>, _: &mut Context<Self>) -> Self::Result {
        self.0.get_vec(&msg.key).cloned()
    }
}

pub fn push<V>(v: V) -> Push<V> {
    Push(v)
}

pub fn pop<V>() -> Pop<V> {
    Pop(PhantomData)
}

pub fn push_front<V>(v: V) -> PushFront<V> {
    PushFront(v)
}

pub fn push_back<V>(v: V) -> PushBack<V> {
    PushBack(v)
}

pub fn pop_front<V>() -> PopFront<V> {
    PopFront(PhantomData)
}

pub fn pop_back<V>() -> PopBack<V> {
    PopBack(PhantomData)
}

pub fn len() -> Len {
    Len
}

pub fn contains<V>(v: V) -> Contains<V> {
    Contains(v)
}

pub fn insert<K, V>(k: K, v: V) -> Insert<K, V> {
    Insert { key: k, value: v }
}

pub fn remove<K, V>(k: K) -> Remove<K, V> {
    Remove {
        key: k,
        value: PhantomData,
    }
}

pub fn remove_vec<K, V>(k: K) -> RemoveVec<K, V> {
    RemoveVec {
        key: k,
        value: PhantomData,
    }
}

pub fn get<K, V>(k: K) -> Get<K, V> {
    Get {
        key: k,
        value: PhantomData,
    }
}

pub fn get_vec<K, V>(k: K) -> GetVec<K, V> {
    GetVec {
        key: k,
        value: PhantomData,
    }
}

pub fn insert_vec<V>(index: usize, v: V) -> Insert<usize, V> {
    Insert {
        key: index,
        value: v,
    }
}

pub use insert_vec as insert_vec_deque;

pub fn insert_set<V>(v: V) -> Insert<(), V> {
    Insert { key: (), value: v }
}
