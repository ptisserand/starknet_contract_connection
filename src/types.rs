#[derive(Debug)] pub struct ContextConfig < A : starknet :: accounts ::
ConnectedAccount + Sync >
{
    pub address : starknet :: core :: types :: FieldElement, pub account : A,
    pub block_id : starknet :: core :: types :: BlockId,
} impl < A : starknet :: accounts :: ConnectedAccount + Sync > ContextConfig <
A >
{
    pub fn
    new(address : starknet :: core :: types :: FieldElement, account : A) ->
    Self
    {
        Self
        {
            address, account, block_id : starknet :: core :: types :: BlockId
            :: Tag(starknet :: core :: types :: BlockTag :: Pending)
        }
    } pub fn
    set_contract_address(mut self, address : starknet :: core :: types ::
    FieldElement) { self.address = address; } pub fn provider(& self) -> & A
    :: Provider { self.account.provider() } pub fn
    set_block(mut self, block_id : starknet :: core :: types :: BlockId)
    { self.block_id = block_id; }
} #[derive(Debug)] pub struct ContextConfigReader < P : starknet :: providers
:: Provider + Sync >
{
    pub address : starknet :: core :: types :: FieldElement, pub provider : P,
    pub block_id : starknet :: core :: types :: BlockId,
} impl < P : starknet :: providers :: Provider + Sync > ContextConfigReader <
P >
{
    pub fn
    new(address : starknet :: core :: types :: FieldElement, provider : P,) ->
    Self
    {
        Self
        {
            address, provider, block_id : starknet :: core :: types :: BlockId
            :: Tag(starknet :: core :: types :: BlockTag :: Pending)
        }
    } pub fn
    set_contract_address(mut self, address : starknet :: core :: types ::
    FieldElement) { self.address = address; } pub fn provider(& self) -> & P
    { & self.provider } pub fn
    set_block(mut self, block_id : starknet :: core :: types :: BlockId)
    { self.block_id = block_id; }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct MemberRemoved
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for MemberRemoved
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(MemberRemoved { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct CapabilityRevoked
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for CapabilityRevoked
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(CapabilityRevoked { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct MemberAdded
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for MemberAdded
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(MemberAdded { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct ApplicationUpdated
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for ApplicationUpdated
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(ApplicationUpdated { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct Application
{
    pub id : starknet :: core :: types :: FieldElement, pub blob : starknet ::
    core :: types :: FieldElement, pub size : u64, pub source : cainome ::
    cairo_serde :: ByteArray, pub metadata : cainome :: cairo_serde ::
    ByteArray
} impl cainome :: cairo_serde :: CairoSerde for Application
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += starknet :: core :: types ::
        FieldElement :: cairo_serialized_size(& __rust.id); __size += starknet
        :: core :: types :: FieldElement ::
        cairo_serialized_size(& __rust.blob); __size += u64 ::
        cairo_serialized_size(& __rust.size); __size += cainome :: cairo_serde
        :: ByteArray :: cairo_serialized_size(& __rust.source); __size +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.metadata); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.id));
        __out.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(& __rust.blob));
        __out.extend(u64 :: cairo_serialize(& __rust.size));
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.source));
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.metadata)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let id = starknet :: core :: types ::
        FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& id); let blob = starknet :: core :: types ::
        FieldElement :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        starknet :: core :: types :: FieldElement ::
        cairo_serialized_size(& blob); let size = u64 ::
        cairo_deserialize(__felts, __offset) ? ; __offset += u64 ::
        cairo_serialized_size(& size); let source = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& source); let metadata = cainome :: cairo_serde
        :: ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& metadata);
        Ok(Application { id, blob, size, source, metadata })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct
OwnershipTransferStarted
{
    pub previous_owner : cainome :: cairo_serde :: ContractAddress, pub
    new_owner : cainome :: cairo_serde :: ContractAddress
} impl cainome :: cairo_serde :: CairoSerde for OwnershipTransferStarted
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde ::
        ContractAddress :: cairo_serialized_size(& __rust.previous_owner);
        __size += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.new_owner); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.previous_owner));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.new_owner)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let previous_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& previous_owner); let new_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& new_owner);
        Ok(OwnershipTransferStarted { previous_owner, new_owner })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct CapabilityGranted
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for CapabilityGranted
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(CapabilityGranted { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct Signed
{
    pub payload : Vec < starknet :: core :: types :: FieldElement > , pub
    signature :
    (starknet :: core :: types :: FieldElement, starknet :: core :: types ::
    FieldElement)
} impl cainome :: cairo_serde :: CairoSerde for Signed
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += Vec :: < starknet :: core :: types ::
        FieldElement > :: cairo_serialized_size(& __rust.payload); __size += <
        (starknet :: core :: types :: FieldElement, starknet :: core :: types
        :: FieldElement) > :: cairo_serialized_size(& __rust.signature);
        __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialize(& __rust.payload));
        __out.extend(<
        (starknet :: core :: types :: FieldElement, starknet :: core :: types
        :: FieldElement) > :: cairo_serialize(& __rust.signature)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let payload = Vec :: < starknet :: core
        :: types :: FieldElement > :: cairo_deserialize(__felts, __offset) ? ;
        __offset += Vec :: < starknet :: core :: types :: FieldElement > ::
        cairo_serialized_size(& payload); let signature = <
        (starknet :: core :: types :: FieldElement, starknet :: core :: types
        :: FieldElement) > :: cairo_deserialize(__felts, __offset) ? ;
        __offset += <
        (starknet :: core :: types :: FieldElement, starknet :: core :: types
        :: FieldElement) > :: cairo_serialized_size(& signature);
        Ok(Signed { payload, signature })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct
OwnershipTransferred
{
    pub previous_owner : cainome :: cairo_serde :: ContractAddress, pub
    new_owner : cainome :: cairo_serde :: ContractAddress
} impl cainome :: cairo_serde :: CairoSerde for OwnershipTransferred
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde ::
        ContractAddress :: cairo_serialized_size(& __rust.previous_owner);
        __size += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& __rust.new_owner); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.previous_owner));
        __out.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(& __rust.new_owner)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let previous_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& previous_owner); let new_owner = cainome ::
        cairo_serde :: ContractAddress :: cairo_deserialize(__felts, __offset)
        ? ; __offset += cainome :: cairo_serde :: ContractAddress ::
        cairo_serialized_size(& new_owner);
        Ok(OwnershipTransferred { previous_owner, new_owner })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub struct ContextCreated
{ pub message : cainome :: cairo_serde :: ByteArray } impl cainome ::
cairo_serde :: CairoSerde for ContextCreated
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        let mut __size = 0; __size += cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& __rust.message); __size
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        let mut __out : Vec < starknet :: core :: types :: FieldElement > =
        vec! [];
        __out.extend(cainome :: cairo_serde :: ByteArray ::
        cairo_serialize(& __rust.message)); __out
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let mut __offset = __offset; let message = cainome :: cairo_serde ::
        ByteArray :: cairo_deserialize(__felts, __offset) ? ; __offset +=
        cainome :: cairo_serde :: ByteArray ::
        cairo_serialized_size(& message); Ok(ContextCreated { message })
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub enum Event
{
    ContextCreated(ContextCreated), MemberAdded(MemberAdded),
    ApplicationUpdated(ApplicationUpdated),
    CapabilityGranted(CapabilityGranted),
    CapabilityRevoked(CapabilityRevoked), MemberRemoved(MemberRemoved),
    OwnableEvent(Event)
} impl cainome :: cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: ContextCreated(val) => ContextCreated ::
            cairo_serialized_size(val) + 1, Event :: MemberAdded(val) =>
            MemberAdded :: cairo_serialized_size(val) + 1, Event ::
            ApplicationUpdated(val) => ApplicationUpdated ::
            cairo_serialized_size(val) + 1, Event :: CapabilityGranted(val) =>
            CapabilityGranted :: cairo_serialized_size(val) + 1, Event ::
            CapabilityRevoked(val) => CapabilityRevoked ::
            cairo_serialized_size(val) + 1, Event :: MemberRemoved(val) =>
            MemberRemoved :: cairo_serialized_size(val) + 1, Event ::
            OwnableEvent(val) => Event :: cairo_serialized_size(val) + 1, _ =>
            0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: ContextCreated(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(ContextCreated :: cairo_serialize(val)); temp
            }, Event :: MemberAdded(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(MemberAdded :: cairo_serialize(val)); temp
            }, Event :: ApplicationUpdated(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 2usize));
                temp.extend(ApplicationUpdated :: cairo_serialize(val)); temp
            }, Event :: CapabilityGranted(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 3usize));
                temp.extend(CapabilityGranted :: cairo_serialize(val)); temp
            }, Event :: CapabilityRevoked(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 4usize));
                temp.extend(CapabilityRevoked :: cairo_serialize(val)); temp
            }, Event :: MemberRemoved(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 5usize));
                temp.extend(MemberRemoved :: cairo_serialize(val)); temp
            }, Event :: OwnableEvent(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 6usize));
                temp.extend(Event :: cairo_serialize(val)); temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            ContextCreated(ContextCreated ::
            cairo_deserialize(__felts, __offset + 1) ?)), 1usize =>
            Ok(Event ::
            MemberAdded(MemberAdded ::
            cairo_deserialize(__felts, __offset + 1) ?)), 2usize =>
            Ok(Event ::
            ApplicationUpdated(ApplicationUpdated ::
            cairo_deserialize(__felts, __offset + 1) ?)), 3usize =>
            Ok(Event ::
            CapabilityGranted(CapabilityGranted ::
            cairo_deserialize(__felts, __offset + 1) ?)), 4usize =>
            Ok(Event ::
            CapabilityRevoked(CapabilityRevoked ::
            cairo_deserialize(__felts, __offset + 1) ?)), 5usize =>
            Ok(Event ::
            MemberRemoved(MemberRemoved ::
            cairo_deserialize(__felts, __offset + 1) ?)), 6usize =>
            Ok(Event ::
            OwnableEvent(Event :: cairo_deserialize(__felts, __offset + 1)
            ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("ContextCreated").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "ContextCreated"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "ContextCreated", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: ContextCreated(ContextCreated { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MemberAdded").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MemberAdded"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "MemberAdded", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: MemberAdded(MemberAdded { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("ApplicationUpdated").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "ApplicationUpdated"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "ApplicationUpdated", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: ApplicationUpdated(ApplicationUpdated { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("CapabilityGranted").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "CapabilityGranted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "CapabilityGranted", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: CapabilityGranted(CapabilityGranted { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("CapabilityRevoked").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "CapabilityRevoked"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "CapabilityRevoked", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: CapabilityRevoked(CapabilityRevoked { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("MemberRemoved").unwrap_or_else(| _ | panic!
        ("Invalid selector for {}", "MemberRemoved"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let message =
            match cainome :: cairo_serde :: ByteArray ::
            cairo_deserialize(& event.data, data_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "message",
                "MemberRemoved", e)),
            }; data_offset += cainome :: cairo_serde :: ByteArray ::
            cairo_serialized_size(& message); return
            Ok(Event :: MemberRemoved(MemberRemoved { message }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("OwnershipTransferred").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnableEvent(Event ::
            OwnershipTransferred(OwnershipTransferred
            { previous_owner, new_owner })))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("OwnershipTransferStarted").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnableEvent(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted
            { previous_owner, new_owner })))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub enum Capability
{ ManageApplication, ManageMembers } impl cainome :: cairo_serde :: CairoSerde
for Capability
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Capability :: ManageApplication => 1, Capability :: ManageMembers
            => 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Capability :: ManageApplication => usize ::
            cairo_serialize(& 0usize), Capability :: ManageMembers => usize ::
            cairo_serialize(& 1usize), _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize => Ok(Capability :: ManageApplication), 1usize =>
            Ok(Capability :: ManageMembers), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format!
            ("Index not handle for enum {}", "Capability")))
        }
    }
} #[derive(Debug, PartialEq, PartialOrd, Clone)] pub enum Event
{
    OwnershipTransferred(OwnershipTransferred),
    OwnershipTransferStarted(OwnershipTransferStarted)
} impl cainome :: cairo_serde :: CairoSerde for Event
{
    type RustType = Self; const SERIALIZED_SIZE : std :: option :: Option <
    usize > = std :: option :: Option :: None; #[inline] fn
    cairo_serialized_size(__rust : & Self :: RustType) -> usize
    {
        match __rust
        {
            Event :: OwnershipTransferred(val) => OwnershipTransferred ::
            cairo_serialized_size(val) + 1, Event ::
            OwnershipTransferStarted(val) => OwnershipTransferStarted ::
            cairo_serialized_size(val) + 1, _ => 0
        }
    } fn cairo_serialize(__rust : & Self :: RustType) -> Vec < starknet ::
    core :: types :: FieldElement >
    {
        match __rust
        {
            Event :: OwnershipTransferred(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 0usize));
                temp.extend(OwnershipTransferred :: cairo_serialize(val));
                temp
            }, Event :: OwnershipTransferStarted(val) =>
            {
                let mut temp = vec! [];
                temp.extend(usize :: cairo_serialize(& 1usize));
                temp.extend(OwnershipTransferStarted :: cairo_serialize(val));
                temp
            }, _ => vec! []
        }
    } fn
    cairo_deserialize(__felts : & [starknet :: core :: types :: FieldElement],
    __offset : usize) -> cainome :: cairo_serde :: Result < Self :: RustType >
    {
        let __index : u128 = __felts [__offset].try_into().unwrap(); match
        __index as usize
        {
            0usize =>
            Ok(Event ::
            OwnershipTransferred(OwnershipTransferred ::
            cairo_deserialize(__felts, __offset + 1) ?)), 1usize =>
            Ok(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted ::
            cairo_deserialize(__felts, __offset + 1) ?)), _ => return
            Err(cainome :: cairo_serde :: Error ::
            Deserialize(format! ("Index not handle for enum {}", "Event")))
        }
    }
} impl TryFrom < starknet :: core :: types :: EmittedEvent > for Event
{
    type Error = String; fn
    try_from(event : starknet :: core :: types :: EmittedEvent) -> Result <
    Self, Self :: Error >
    {
        use cainome :: cairo_serde :: CairoSerde; if event.keys.is_empty()
        { return Err("Event has no key".to_string()); } let selector =
        event.keys [0]; if selector == starknet :: core :: utils ::
        get_selector_from_name("OwnershipTransferred").unwrap_or_else(| _ |
        panic! ("Invalid selector for {}", "OwnershipTransferred"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferred", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnershipTransferred(OwnershipTransferred
            { previous_owner, new_owner }))
        }; let selector = event.keys [0]; if selector == starknet :: core ::
        utils ::
        get_selector_from_name("OwnershipTransferStarted").unwrap_or_else(| _
        | panic! ("Invalid selector for {}", "OwnershipTransferStarted"))
        {
            let mut key_offset = 0 + 1; let mut data_offset = 0; let
            previous_owner = match cainome :: cairo_serde :: ContractAddress
            :: cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}",
                "previous_owner", "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& previous_owner); let new_owner = match
            cainome :: cairo_serde :: ContractAddress ::
            cairo_deserialize(& event.keys, key_offset)
            {
                Ok(v) => v, Err(e) => return
                Err(format!
                ("Could not deserialize field {} for {}: {:?}", "new_owner",
                "OwnershipTransferStarted", e)),
            }; key_offset += cainome :: cairo_serde :: ContractAddress ::
            cairo_serialized_size(& new_owner); return
            Ok(Event ::
            OwnershipTransferStarted(OwnershipTransferStarted
            { previous_owner, new_owner }))
        };
        Err(format! ("Could not match any event from keys {:?}", event.keys))
    }
} impl < A : starknet :: accounts :: ConnectedAccount + Sync > ContextConfig <
A >
{
    #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub fn
    owner(& self,) -> cainome :: cairo_serde :: call :: FCall < A :: Provider,
    cainome :: cairo_serde :: ContractAddress >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("owner"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    application(& self, context_id : & starknet :: core :: types ::
    FieldElement) -> cainome :: cairo_serde :: call :: FCall < A :: Provider,
    Application >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("application"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    members(& self, context_id : & starknet :: core :: types :: FieldElement,
    offset : & u32, length : & u32) -> cainome :: cairo_serde :: call :: FCall
    < A :: Provider, Vec :: < starknet :: core :: types :: FieldElement > >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(u32 :: cairo_serialize(offset));
        __calldata.extend(u32 :: cairo_serialize(length)); let __call =
        starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("members"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    privileges(& self, context_id : & starknet :: core :: types ::
    FieldElement, identities : & Vec :: < starknet :: core :: types ::
    FieldElement >) -> cainome :: cairo_serde :: call :: FCall < A ::
    Provider, Vec :: <
    (starknet :: core :: types :: FieldElement, Vec :: < Capability >) > >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(identities)); let __call = starknet :: core ::
        types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("privileges"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    get_member_nonce(& self, context_id : & starknet :: core :: types ::
    FieldElement, member_id : & starknet :: core :: types :: FieldElement) ->
    cainome :: cairo_serde :: call :: FCall < A :: Provider, u64 >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(member_id)); let __call = starknet :: core :: types ::
        FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_member_nonce"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    transfer_ownership_getcall(& self, new_owner : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(new_owner)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("transfer_ownership"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    transfer_ownership(& self, new_owner : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(new_owner)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("transfer_ownership"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn renounce_ownership_getcall(& self,) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("renounce_ownership"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn renounce_ownership(& self,) ->
    starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("renounce_ownership"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    transferOwnership_getcall(& self, newOwner : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(newOwner)); starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("transferOwnership"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    transferOwnership(& self, newOwner : & cainome :: cairo_serde ::
    ContractAddress) -> starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(cainome :: cairo_serde :: ContractAddress ::
        cairo_serialize(newOwner)); let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("transferOwnership"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn renounceOwnership_getcall(& self,) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("renounceOwnership"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn renounceOwnership(& self,) ->
    starknet :: accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("renounceOwnership"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn mutate_getcall(& self, signed_request : & Signed) -> starknet ::
    accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; __calldata.extend(Signed :: cairo_serialize(signed_request));
        starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("mutate"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn
    mutate(& self, signed_request : & Signed) -> starknet :: accounts ::
    Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; __calldata.extend(Signed :: cairo_serialize(signed_request)); let
        __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("mutate"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn erase_getcall(& self,) -> starknet :: accounts :: Call
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("erase"), calldata : __calldata,
        }
    } #[allow(clippy :: ptr_arg)] pub fn erase(& self,) -> starknet ::
    accounts :: Execution < A >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: accounts :: Call
        {
            to : self.address, selector : starknet :: macros :: selector!
            ("erase"), calldata : __calldata,
        }; self.account.execute(vec! [__call])
    }
} impl < P : starknet :: providers :: Provider + Sync > ContextConfigReader <
P >
{
    #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub fn
    owner(& self,) -> cainome :: cairo_serde :: call :: FCall < P, cainome ::
    cairo_serde :: ContractAddress >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        []; let __call = starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("owner"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    application(& self, context_id : & starknet :: core :: types ::
    FieldElement) -> cainome :: cairo_serde :: call :: FCall < P, Application
    >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id)); let __call = starknet :: core :: types
        :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("application"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    members(& self, context_id : & starknet :: core :: types :: FieldElement,
    offset : & u32, length : & u32) -> cainome :: cairo_serde :: call :: FCall
    < P, Vec :: < starknet :: core :: types :: FieldElement > >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(u32 :: cairo_serialize(offset));
        __calldata.extend(u32 :: cairo_serialize(length)); let __call =
        starknet :: core :: types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("members"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    privileges(& self, context_id : & starknet :: core :: types ::
    FieldElement, identities : & Vec :: < starknet :: core :: types ::
    FieldElement >) -> cainome :: cairo_serde :: call :: FCall < P, Vec :: <
    (starknet :: core :: types :: FieldElement, Vec :: < Capability >) > >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(Vec :: < starknet :: core :: types :: FieldElement >
        :: cairo_serialize(identities)); let __call = starknet :: core ::
        types :: FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("privileges"), calldata : __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    } #[allow(clippy :: ptr_arg)] #[allow(clippy :: too_many_arguments)] pub
    fn
    get_member_nonce(& self, context_id : & starknet :: core :: types ::
    FieldElement, member_id : & starknet :: core :: types :: FieldElement) ->
    cainome :: cairo_serde :: call :: FCall < P, u64 >
    {
        use cainome :: cairo_serde :: CairoSerde; let mut __calldata = vec!
        [];
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(context_id));
        __calldata.extend(starknet :: core :: types :: FieldElement ::
        cairo_serialize(member_id)); let __call = starknet :: core :: types ::
        FunctionCall
        {
            contract_address : self.address, entry_point_selector : starknet
            :: macros :: selector! ("get_member_nonce"), calldata :
            __calldata,
        }; cainome :: cairo_serde :: call :: FCall ::
        new(__call, self.provider(),)
    }
}