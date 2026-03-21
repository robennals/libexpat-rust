# Comprehensive Architecture Analysis: libexpat xmlparse.c

**Source**: `/Users/robennals/broomer-repos/libexpat-rust/port/start/expat/lib/xmlparse.c`
**Size**: 9,267 lines
**Purpose**: Complete C parser for XML with DTD, entities, namespaces, and security features

---

## 1. Key Data Structures

### Core Parser: `XML_ParserStruct` (lines 665-787)
Main parser state container with ~70 fields:

**Parse Buffer Management**:
- `m_buffer`: Malloc/realloc base pointer
- `m_bufferPtr`: First character to parse (marks parsed boundary)
- `m_bufferEnd`: Past last character to parse (marks parsing boundary)
- `m_bufferLim`: Allocated end of buffer
- `m_parseEndByteIndex`: Byte index of parse end
- `m_parseEndPtr`: Pointer to parse end
- Invariant: `m_buffer ≤ m_bufferPtr ≤ m_bufferEnd ≤ m_bufferLim`

**Callback Handlers** (20+ function pointers):
- Element: `m_startElementHandler`, `m_endElementHandler`
- Text: `m_characterDataHandler`, `m_defaultHandler`
- DTD: `m_startDoctypeDeclHandler`, `m_endDoctypeDeclHandler`, `m_elementDeclHandler`, `m_attlistDeclHandler`, `m_entityDeclHandler`
- Entities: `m_externalEntityRefHandler`, `m_skippedEntityHandler`, `m_unparsedEntityDeclHandler`
- Formatting: `m_processingInstructionHandler`, `m_commentHandler`
- CDATA: `m_startCdataSectionHandler`, `m_endCdataSectionHandler`
- Namespaces: `m_startNamespaceDeclHandler`, `m_endNamespaceDeclHandler`
- Declarations: `m_xmlDeclHandler`, `m_notationDeclHandler`
- Other: `m_unknownEncodingHandler`, `m_notStandaloneHandler`
- User data: `m_userData`, `m_handlerArg`

**Parser State Machine**:
- `m_processor`: Function pointer to current processor (state machine dispatch)
- `m_prologState`: Current prolog parsing state
- `m_errorCode`: Last error code
- `m_parsingStatus`: Parsing state (INITIALIZED, PARSING, SUSPENDED, FINISHED)
- `m_reenter`: Reentrancy guard

**Event Tracking**:
- `m_eventPtr`, `m_eventEndPtr`: Current event position
- `m_positionPtr`: Current character position for line/col calculation
- `m_position`: Line/column counters

**Encoding**:
- `m_encoding`: Current encoding (from document or declared)
- `m_internalEncoding`: Internal representation encoding
- `m_initEncoding`: Initial encoding before document declaration
- `m_protocolEncodingName`: Encoding from transport layer
- `m_unknownEncodingMem/Data/Release`: Unknown encoding handler state

**Element Stack**:
- `m_tagStack`: Currently open elements
- `m_freeTagList`: Reusable tag structures
- `m_tagLevel`: Nesting depth
- `m_inheritedBindings`: Inherited namespace bindings
- `m_freeBindingList`: Reusable binding structures

**DTD**:
- `m_dtd`: Pointer to document type definition
- Various DTD-related flags

**Namespace**:
- `m_ns`: Enable namespace processing
- `m_ns_triplets`: Expand namespace triplets
- `m_namespaceSeparator`: Character separating namespace:local

**Attributes**:
- `m_atts`: Attribute array
- `m_attsSize`: Allocated attribute slots
- `m_nSpecifiedAtts`: Number of actual attributes
- `m_idAttIndex`: ID attribute index if present
- `m_nsAtts`: Namespace attribute hash
- `m_attInfo`: Attribute position information (when XML_ATTR_INFO enabled)

**Declaration Context**:
- `m_declEntity`: Currently declared entity
- `m_declElementType`: Currently declared element
- `m_declAttributeId`, `m_declAttributeType`: Currently declared attribute
- `m_declAttributeIsCdata`, `m_declAttributeIsId`: Attribute flags
- `m_declNotationName`, `m_declNotationPublicId`: Currently declared notation

**DOCTYPE Context**:
- `m_doctypeName`: Root element name
- `m_doctypeSysid`: System identifier
- `m_doctypePubid`: Public identifier

**Pools for Accumulating Data**:
- `m_dataBuf/m_dataBufEnd`: Character data buffer
- `m_tempPool`: Temporary string accumulation
- `m_temp2Pool`: Secondary temporary pool
- `m_curBase`: Current base URI

**Other**:
- `m_parentParser`: Parent parser for external entities
- `m_isParamEntity`: This is a parameter entity parser
- `m_useForeignDTD`: Use external DTD flag
- `m_paramEntityParsing`: Parameter entity parsing mode
- `m_defaultExpandInternalEntities`: Auto-expand internal entities
- `m_hash_secret_salt`: Hash randomization
- Security structures (when `XML_GE==1`): `m_accounting`, `m_alloc_tracker`, `m_entity_stats`

---

### DTD (Document Type Definition) (lines 393-420)

```c
typedef struct {
  HASH_TABLE generalEntities;      // General entity declarations
  HASH_TABLE elementTypes;         // Element type definitions
  HASH_TABLE attributeIds;         // Attribute ID definitions
  HASH_TABLE prefixes;             // Namespace prefix mappings
  STRING_POOL pool;                // Name/value storage
  STRING_POOL entityValuePool;     // Entity expansion text storage
  XML_Bool keepProcessing;         // False once PE ref skipped
  XML_Bool hasParamEntityRefs;     // True if PE referenced
  XML_Bool standalone;             // Standalone declaration flag
  XML_Bool paramEntityRead;        // (XML_DTD) External PE read
  HASH_TABLE paramEntities;        // (XML_DTD) Parameter entities
  PREFIX defaultPrefix;            // Default namespace prefix

  // Content model scaffolding for building element content specs
  XML_Bool in_eldecl;
  CONTENT_SCAFFOLD *scaffold;
  unsigned contentStringLen;
  unsigned scaffSize;
  unsigned scaffCount;
  int scaffLevel;
  int *scaffIndex;
} DTD;
```

**Purpose**: Centralizes all DTD information. Created on parser construction, reset on parser reset, destroyed on parser free.

---

### Hash Table: `HASH_TABLE` (lines 229-235)

```c
typedef struct {
  NAMED **v;           // Array of pointers to hash entries
  unsigned char power; // log2(size)
  size_t size;         // 2^power
  size_t used;         // Number of entries
  XML_Parser parser;   // For memory allocation
} HASH_TABLE;
```

**Algorithm**: Double-hashing with power-of-2 size
- Primary hash: `index = hash & mask` (where `mask = size - 1`)
- Secondary hash: `SECOND_HASH(hash, mask, power)` for collision resolution
- Probe step: `PROBE_STEP(hash, mask, power)` (always odd for relative primality)
- Used for entities, element types, attributes, prefixes

---

### String Pool: `STRING_POOL` (lines 354-361)

```c
typedef struct {
  BLOCK *blocks;          // Linked list of allocated blocks
  BLOCK *freeBlocks;      // Free blocks for reuse
  const XML_Char *end;    // End of current block
  XML_Char *ptr;          // Current write pointer
  XML_Char *start;        // Start of string being built
  XML_Parser parser;      // For memory allocation
} STRING_POOL;

typedef struct block {
  struct block *next;
  int size;
  XML_Char s[];           // Flexible array member
} BLOCK;
```

**Purpose**: Efficient allocation of interned strings. Strings are stored once, referenced everywhere (reduces memory duplication).

---

### Entities: `ENTITY` (lines 318-334)

```c
typedef struct {
  const XML_Char *name;       // Entity name
  const XML_Char *textPtr;    // Expanded text (internal entities)
  int textLen;                // Length in XML_Chars
  int processed;              // Bytes processed when suspended
  const XML_Char *systemId;   // External entity system ID
  const XML_Char *base;       // Base URI for resolution
  const XML_Char *publicId;   // External entity public ID
  const XML_Char *notation;   // Unparsed entity notation
  XML_Bool open;              // Currently being parsed
  XML_Bool hasMore;           // More text to process
  XML_Bool is_param;          // Parameter entity flag
  XML_Bool is_internal;       // Declared in internal subset
} ENTITY;
```

**Storage**: Stored in `m_dtd->generalEntities` (general) or `m_dtd->paramEntities` (parameter).

---

### Open Entity Stack: `OPEN_INTERNAL_ENTITY` (lines 428-435)

```c
typedef struct open_internal_entity {
  const char *internalEventPtr;          // Current position in entity text
  const char *internalEventEndPtr;       // End of token
  struct open_internal_entity *next;     // Stack link
  ENTITY *entity;                        // Pointer to entity definition
  int startTagLevel;                     // Element depth when opened
  XML_Bool betweenDecl;                  // WFC: PE Between Declarations
  enum EntityType type;                  // INTERNAL, ATTRIBUTE, VALUE
} OPEN_INTERNAL_ENTITY;
```

**Purpose**: Tracks currently-expanding entities. Maintains depth checking to prevent infinite recursion.

---

### Namespace Support

**PREFIX** (lines 278-281):
```c
typedef struct prefix {
  const XML_Char *name;     // Prefix string (e.g., "xsl")
  BINDING *binding;         // Current binding (see below)
} PREFIX;
```

**BINDING** (lines 268-276):
```c
typedef struct binding {
  struct prefix *prefix;                    // Back-pointer to prefix
  struct binding *nextTagBinding;           // Bindings in current tag
  struct binding *prevPrefixBinding;        // Previous binding for this prefix
  const struct attribute_id *attId;         // If xmlns attribute
  XML_Char *uri;                            // Namespace URI
  int uriLen;                               // URI length
  int uriAlloc;                             // Allocated space
} BINDING;
```

**Purpose**: Implements namespace scope as linked lists. New tag pushes new bindings; tag closure pops them.

**TAG_NAME** (lines 283-290):
```c
typedef struct {
  const XML_Char *str;          // Full qualified name
  const XML_Char *localPart;    // Local part after colon
  const XML_Char *prefix;       // Prefix part before colon
  int strLen;                   // String length
  int uriLen;                   // URI length
  int prefixLen;                // Prefix length
} TAG_NAME;
```

---

### Element Stack: `TAG` (lines 305-316)

```c
typedef struct tag {
  struct tag *parent;           // Parent element (for nesting)
  const char *rawName;          // Tag in original encoding
  int rawNameLength;            // Raw name byte length
  TAG_NAME name;                // Tag in API encoding
  union {
    char *raw;                  // Byte-level access
    XML_Char *str;              // Character-level access
  } buf;
  char *bufEnd;                 // End of allocated buffer
  BINDING *bindings;            // Namespace bindings for this element
} TAG;
```

**Purpose**: Represents an open element. Maintains both raw (document encoding) and converted (internal encoding) names. Namespace bindings are local to each tag.

---

### Element Type: `ELEMENT_TYPE` (lines 384-391)

```c
typedef struct {
  const XML_Char *name;                    // Element name
  PREFIX *prefix;                          // Namespace prefix (if NS mode)
  const ATTRIBUTE_ID *idAtt;               // ID attribute if defined
  int nDefaultAtts;                        // Number of defaults
  int allocDefaultAtts;                    // Allocated slots
  DEFAULT_ATTRIBUTE *defaultAtts;          // Default values
} ELEMENT_TYPE;
```

**Purpose**: Defines an element type from `<!ELEMENT>` declarations.

---

### Attribute ID: `ATTRIBUTE_ID` (lines 365-370)

```c
typedef struct attribute_id {
  XML_Char *name;               // Attribute name
  PREFIX *prefix;               // Namespace prefix
  XML_Bool maybeTokenized;      // Could be a token type
  XML_Bool xmlns;               // Is an xmlns attribute
} ATTRIBUTE_ID;
```

---

### Content Model: `CONTENT_SCAFFOLD` (lines 336-344)

```c
typedef struct {
  enum XML_Content_Type type;    // ELEMENT, MIXED, ANY, EMPTY
  enum XML_Content_Quant quant;  // NONE, OPT, REP, PLUS
  const XML_Char *name;          // Element name for ELEMENT type
  int firstchild;                // Index to first child
  int lastchild;                 // Index to last child
  int childcnt;                  // Number of children
  int nextsib;                   // Next sibling index
} CONTENT_SCAFFOLD;
```

**Purpose**: Temporary structure during DTD parsing, converted to `XML_Content` tree after parsing completes.

---

### Security Structures (when `XML_GE == 1`)

**ACCOUNTING** (lines 447-453): Tracks billion-laughs attack defense
```c
typedef struct accounting {
  XmlBigCount countBytesDirect;              // Direct input bytes
  XmlBigCount countBytesIndirect;            // Expanded entity bytes
  unsigned long debugLevel;
  float maximumAmplificationFactor;          // e.g., 100.0
  unsigned long long activationThresholdBytes; // e.g., 8 MiB
} ACCOUNTING;
```

**MALLOC_TRACKER** (lines 455-461): Allocation limit enforcement
```c
typedef struct MALLOC_TRACKER {
  XmlBigCount bytesAllocated;
  XmlBigCount peakBytesAllocated;
  unsigned long debugLevel;
  float maximumAmplificationFactor;
  XmlBigCount activationThresholdBytes;      // e.g., 64 MiB
} MALLOC_TRACKER;
```

**ENTITY_STATS** (lines 463-468): Entity nesting tracking
```c
typedef struct entity_stats {
  unsigned int countEverOpened;
  unsigned int currentDepth;
  unsigned int maximumDepthSeen;
  unsigned long debugLevel;
} ENTITY_STATS;
```

---

## 2. Parser State Machine & Processors

### Processor Function Type (lines 471-472)

```c
typedef enum XML_Error PTRCALL Processor(XML_Parser parser, const char *start,
                                         const char *end, const char **endPtr);
```

**Semantics**:
- Returns `XML_ERROR_NONE` on success
- On success, sets `*endPtr` to point past last processed byte
- On error, returns error code and parser enters error state
- Function pointer in `parser->m_processor` implements state machine

---

### Processor Functions & State Flow

```
XML_Parse/XML_ParseBuffer
    ↓
    ├─ [XML_INITIALIZED] → startParsing() → m_processor = prologInitProcessor
    └─ [XML_PARSING] → callProcessor()

prologInitProcessor (line 4987)
    ├─ initializeEncoding()
    └─ m_processor = prologProcessor
        ↓
        prologProcessor (line 5196)
            └─ doProlog() — Parse XML declaration, DTD, process declarations
                ├─ Handles: <?xml>, <!DOCTYPE>, <!ELEMENT>, <!ATTLIST>,
                │           <!ENTITY>, <!NOTATION>, etc.
                └─ When prolog done → m_processor = contentProcessor
                    ↓
                    contentProcessor (line 3179)
                        └─ doContent() — Parse element content
                            ├─ Handles: start tags, end tags, text, references
                            ├─ On <: storeAtts(), invoke m_startElementHandler
                            ├─ On </: invoke m_endElementHandler
                            ├─ On text: invoke m_characterDataHandler
                            └─ On CDATA: m_processor = cdataSectionProcessor
                                ↓
                                cdataSectionProcessor
                                    └─ doCdataSection()
                                        └─ When CDATA closes → contentProcessor

                    When root element closes and prolog complete:
                        → m_processor = epilogProcessor
                            └─ Handles content after root (should be empty)

External Entity Handling:
    externalEntityInitProcessor (line 3193)
        ├─ initializeEncoding()
        └─ m_processor = externalEntityInitProcessor2
            ↓
            externalEntityInitProcessor2 (line 3203)
                ├─ Detects & skips BOM
                └─ m_processor = externalEntityInitProcessor3
                    ↓
                    externalEntityInitProcessor3 (line 3248)
                        ├─ Processes optional XML declaration
                        └─ m_processor = externalEntityContentProcessor
                            ↓
                            externalEntityContentProcessor (line 3300)
                                └─ doContent() with XML_ACCOUNT_ENTITY_EXPANSION

Parameter Entity Handling (XML_DTD):
    externalParEntInitProcessor (line 4999)
        ├─ initializeEncoding()
        └─ If inEntityValue → entityValueInitProcessor
           Else → externalParEntProcessor

Entity Value Processing (XML_DTD):
    entityValueInitProcessor (line 5019)
        └─ Scans for entity value end
        └─ m_processor = entityValueProcessor
            └─ storeEntityValue()

Error Handling:
    [any processor detects error]
        └─ m_processor = errorProcessor
            └─ All subsequent calls return same error
```

---

### Core Processing Functions

**doProlog** (lines 5206-6280): ~1000 lines, main DTD/declaration parser
- Tokenizes with `XmlPrologTok()`
- Uses `XmlTokenRole()` for syntax validation
- Handles:
  - `<?xml version=... encoding=... standalone=...?>`
  - `<!DOCTYPE name ... >`
  - `<!ELEMENT name contentspec>`
  - `<!ATTLIST name attdefs>`
  - `<!ENTITY name ... >`
  - `<!NOTATION name ...>`
  - Comments, PIs
  - Parameter entity references
- Sets `parser->m_processor = contentProcessor` when prolog done

**doContent** (lines 3314-3828): ~500 lines, main element/text parser
- Tokenizes with `XmlContentTok()`
- Handles:
  - Start tags: `storeAtts()`, `m_startElementHandler`
  - End tags: `m_endElementHandler`
  - Text data: `m_characterDataHandler`
  - Entity references: `processEntity()`
  - CDATA sections: switch to `cdataSectionProcessor`
  - PIs, comments: report via handlers
- Maintains element stack via `m_tagStack`

**doCdataSection** (lines 3830-3900): CDATA section parser
- Scans for `]]>` terminator
- Accumulates text in pool
- Invokes `m_startCdataSectionHandler` / `m_endCdataSectionHandler`

**doIgnoreSection** (lines 3902-3950, XML_DTD): IGNORE section parser
- Used in conditional sections
- Counts nesting of `<![` and `]]>`

**processXmlDecl** (lines 4410-4570): XML declaration handler
- Parses `version`, `encoding`, `standalone`
- Calls `m_xmlDeclHandler` if installed
- May trigger encoding change

**initializeEncoding** (lines 3313): Encoding setup
- Called at start of parsing
- Determines encoding from BOM, declaration, or default

---

## 3. Memory Management

### String Pool Operations

- **poolInit(pool, parser)** (line 583): Initialize empty pool
- **poolAppend(pool, enc, ptr, end)** (line 586): Add bytes to current string
- **poolStoreString(pool, enc, ptr, end)** (line 588): Store bytes as new string, return pointer
- **poolCopyString(pool, s)** (line 591): Copy existing string to pool
- **poolCopyStringN(pool, s, n)** (line 595): Copy first N characters
- **poolAppendString(pool, s)** (line 597): Append string to current string
- **poolCopyStringNoFinish(pool, s)** (line 593): Copy without finishing current string
- **poolGrow(pool)** (line 590): Expand current block
- **poolClear(pool)** (line 584): Reset pool (clear m_ptr)
- **poolDiscard(pool)** macro: Discard current string without finishing
- **poolChop(pool)** macro: Remove last character

**Macros for access**:
- `poolStart(pool)`: First character of current string
- `poolLength(pool)`: Bytes in current string
- `poolFinish(pool)`: Mark end of current string
- `poolLastChar(pool)`: Get last character

---

### Hash Table Operations

- **hashTableInit(table, parser)** (line 576): Initialize empty table
- **hashTableClear(table)** (line 577): Clear all entries
- **hashTableDestroy(table)** (line 578): Free table
- **lookup(parser, table, name, createSize)** (line 574): Find/create entry
  - Returns `NAMED*` (base structure with KEY name field)
  - Creates with `createSize` bytes if not found
  - Double-hashing collision resolution
  - Auto-grows when load factor exceeded
- **hashTableIterInit(iter, table)** (line 579): Initialize iterator
- **hashTableIterNext(iter)** (line 581): Next entry, returns NULL at end

**Algorithm Details**:
```c
#define SECOND_HASH(hash, mask, power) \
  ((((hash) & ~(mask)) >> ((power) - 1)) & ((mask) >> 2))
#define PROBE_STEP(hash, mask, power) \
  ((unsigned char)((SECOND_HASH(hash, mask, power)) | 1))
```
- Guarantees step is odd (relative prime to power-of-2 table size)
- Maximum step size is `table->size / 4`

---

### DTD Lifecycle

- **dtdCreate(parser)** (line 566): Create new DTD
  - Initializes all hash tables and pools
  - Called during XML_ParserCreate via parserCreate()
- **dtdReset(dtd, parser)** (line 568): Reset for reuse
  - Clears entities, element types, attributes
  - Called by XML_ParserReset()
- **dtdDestroy(dtd, isDocEntity, parser)** (line 569): Free DTD
  - Frees all tables and pools
  - Called by XML_ParserFree()
- **dtdCopy(oldParser, newDtd, oldDtd, parser)** (line 570): Copy DTD
  - For XML_ParserReset() to preserve DTD if desired
  - Copies entity table, element types, etc.
- **copyEntityTable(oldParser, newTable, newPool, oldTable)** (line 572): Copy one table

---

### Entity/Tag Lifecycle

- **Free lists maintain reusable structures**:
  - `parser->m_freeTagList`: Reusable TAG structures
  - `parser->m_freeInternalEntities`: Reusable OPEN_INTERNAL_ENTITY
  - `parser->m_freeBindingList`: Reusable BINDING structures
  - `parser->m_freeAttributeEntities`, `parser->m_freeValueEntities`

- **Allocation on demand**:
  - When needed, allocate new
  - When done, link to free list
  - Reduces malloc/free churn

---

### Security: Amplification Tracking

When `XML_GE == 1`:

- **accountingGetCurrentAmplification(parser)** (line 618): Compute `bytes_indirect / bytes_direct`
- **accountingDiffTolerated(parser, tok, before, after, ...)** (line 626): Check if expansion within limit
  - Called from every tokenization point
  - Returns `XML_FALSE` if limit exceeded
  - Sets error to `XML_ERROR_AMPLIFICATION_LIMIT_BREACH`
- **accountingReportDiff(parser, ...)** (line 621): Update counters
- **accountingOnAbort(parser)** (line 620): Cleanup on error
- **entityTrackingOnOpen/OnClose/ReportStats** (lines 633-636): Track entity nesting

**Activation threshold**: Only enforced after X bytes processed (default 8 MiB).

---

## 4. Callback System

### Callback Handler Storage
All stored as function pointers in `XML_ParserStruct`, with 3 contexts:

**Element Context**:
```
m_startElementHandler(handlerArg, elname, atts)    // Start tag
m_endElementHandler(handlerArg, elname)             // End tag
m_startNamespaceDeclHandler(handlerArg, prefix, uri) // NS decl
m_endNamespaceDeclHandler(handlerArg, prefix)        // NS end
```

**Content Context**:
```
m_characterDataHandler(handlerArg, data, len)       // Text
m_processingInstructionHandler(handlerArg, target, data)
m_commentHandler(handlerArg, data)
```

**CDATA Context**:
```
m_startCdataSectionHandler(handlerArg)
m_endCdataSectionHandler(handlerArg)
```

**DTD Context**:
```
m_startDoctypeDeclHandler(handlerArg, doctypeName, sysId, pubId, hasIntSubset)
m_endDoctypeDeclHandler(handlerArg)
m_elementDeclHandler(handlerArg, name, model)
m_attlistDeclHandler(handlerArg, elname, attname, type, dflt, isRequired)
m_entityDeclHandler(handlerArg, entityName, isParameterEntity, value, valueLen,
                    sysId, pubId, notationName)
m_notationDeclHandler(handlerArg, notationName, sysId, pubId)
m_unparsedEntityDeclHandler(handlerArg, entityName, sysId, pubId, notationName)
```

**Other Contexts**:
```
m_xmlDeclHandler(handlerArg, version, encoding, standalone)
m_externalEntityRefHandler(parser, context, base, sysId, pubId)  // Special: gets parser
m_unknownEncodingHandler(encodingHandlerData, name, info)        // Special: encoding
m_defaultHandler(handlerArg, data, len)                          // Catch-all
m_notStandaloneHandler(handlerArg)
m_skippedEntityHandler(handlerArg, entityName, isParameterEntity)
```

### Handler Invocation Pattern

**User data context**:
- `m_userData`: User-supplied opaque pointer (via `XML_SetUserData`)
- `m_handlerArg`: Actual argument passed to handlers (usually `m_userData`, but can differ for external entities)

**Example**:
```c
if (parser->m_startElementHandler) {
  parser->m_startElementHandler(parser->m_handlerArg, tag_name, attributes);
}
```

### Helper Functions for Common Patterns

- **reportProcessingInstruction(parser, enc, start, end)** (line 554): Invoke PI handler
- **reportComment(parser, enc, start, end)** (line 556): Invoke comment handler
- **reportDefault(parser, enc, start, end)** (line 558): Invoke default handler
- These decode from source encoding to API encoding

---

## 5. Entity Handling

### Entity Declaration & Storage

**Types**:
- `ENTITY_INTERNAL`: Declared in document (general or parameter entity)
- `ENTITY_ATTRIBUTE`: Referenced in attribute value
- `ENTITY_VALUE`: Referenced in entity value

**Storage Locations**:
- General entities: `parser->m_dtd->generalEntities` hash table
- Parameter entities: `parser->m_dtd->paramEntities` hash table (XML_DTD only)
- Access via `lookup()` function

**Fields**:
```c
ENTITY {
  name                // Entity name (e.g., "copy")
  textPtr             // Expanded text pointer
  textLen             // Text length
  systemId, publicId  // External entity references
  base                // Base URI for external resolution
  notation            // For unparsed entities
  open                // Currently being parsed
  hasMore             // More text to process
  is_param            // Parameter entity flag
  is_internal         // In internal subset (not external)
}
```

### Entity Expansion Stack

**Open Entity Tracking** via `OPEN_INTERNAL_ENTITY`:
```
m_openInternalEntities      // Stack of currently-expanding entities
m_freeInternalEntities      // Free list for reuse

m_openAttributeEntities     // Stack for entities in attribute values
m_freeAttributeEntities

m_openValueEntities         // Stack for entities in entity values
m_freeValueEntities
```

**Stack Structure**:
- Linked list (singly-linked via `next` pointer)
- Latest opened entity at head
- Tracks:
  - `internalEventPtr/internalEventEndPtr`: Current position in entity text
  - `entity`: Pointer to ENTITY definition
  - `startTagLevel`: Element nesting depth when opened
  - `betweenDecl`: WFC check for parameter entities between declarations
  - `type`: Which stack this came from

### Entity Processing

**processEntity(parser, entity, betweenDecl, type)** (line 503):
- Main entity expansion handler
- Allocates new open entity structure
- Pushes onto appropriate stack
- Sets parsing to internal encoding
- Returns to parser to continue

**Entity recursion prevention**:
- Each entity tracks `startTagLevel` (depth when opened)
- Recursive reference checked: if nesting depth == starting depth, likely infinite recursion
- Error `XML_ERROR_ASYNC_ENTITY` if closing tag at wrong level

**Default entity expansion**:
- `m_defaultExpandInternalEntities`: Control expansion of built-in entities
- Built-in entities: `&lt;`, `&gt;`, `&amp;`, `&quot;`, `&apos;`

---

## 6. DTD Processing

### DTD Parsing: doProlog

**Main state machine** (lines 5206-6280, ~1000 lines):
- Uses `XmlPrologTok()` to tokenize
- Uses `XmlTokenRole()` for validation
- Processes declarations in order
- Maintains `PROLOG_STATE` via `XmlTokenRole()` callbacks

**Declarations Handled**:

1. **XML Declaration** `<?xml ...?>`:
   - Version, encoding, standalone
   - Calls `processXmlDecl()`
   - Triggers encoding change if needed

2. **DOCTYPE Declaration** `<!DOCTYPE name ...>`:
   - External subset reference (system/public IDs)
   - Internal subset in brackets
   - Tracked in `m_doctypeName/Sysid/Pubid`
   - Calls `m_startDoctypeDeclHandler` / `m_endDoctypeDeclHandler`

3. **Element Declaration** `<!ELEMENT name contentspec>`:
   - Content spec types: EMPTY, ANY, (model)
   - Calls `m_elementDeclHandler`
   - Builds `CONTENT_SCAFFOLD` temporarily
   - Converts to `XML_Content` tree at end
   - Stores in `m_dtd->elementTypes` hash table

4. **Attribute List Declaration** `<!ATTLIST name attdefs>`:
   - Multiple attributes per declaration
   - Calls `m_attlistDeclHandler` for each
   - Creates `ATTRIBUTE_ID` structures
   - Stores in `m_dtd->attributeIds`
   - Links to `ELEMENT_TYPE`

5. **Entity Declaration** `<!ENTITY name ... >`:
   - General entities: `<!ENTITY copy "©">`
   - Parameter entities: `<!ENTITY % name ...>` (XML_DTD)
   - External entities: system/public IDs
   - Calls `m_entityDeclHandler`
   - Stores in `m_dtd->generalEntities` or `m_dtd->paramEntities`

6. **Notation Declaration** `<!NOTATION name ...>`:
   - For unparsed entities
   - Calls `m_notationDeclHandler`
   - Stored in separate notation hash table

7. **Parameter Entity References** `%name;`:
   - Only in DTD context
   - Triggers `processEntity()` to expand
   - Can be conditional (`<![ INCLUDE [ ... ]]>`, `<![ IGNORE [ ... ]]>`)

8. **Comments** `<!-- ... -->`:
   - Calls `m_commentHandler`

9. **Processing Instructions** `<?target ...?>`:
   - Calls `m_processingInstructionHandler`

---

### Element Type Management

**ELEMENT_TYPE Structure**:
```c
{
  const XML_Char *name;              // Element name
  PREFIX *prefix;                    // NS prefix (if NS mode)
  const ATTRIBUTE_ID *idAtt;         // ID attribute if any
  int nDefaultAtts;                  // Number of defaults
  int allocDefaultAtts;              // Allocated slots
  DEFAULT_ATTRIBUTE *defaultAtts;    // Array of defaults
}
```

**Functions**:
- **getElementType(parser, enc, ptr, end)** (line 602): Find/create element type
- **defineAttribute(type, attId, isCdata, isId, value, parser)** (line 527): Add attribute to type
- **setElementTypePrefix(parser, elementType)** (line 541): Set namespace prefix

**Default Attributes**:
- `DEFAULT_ATTRIBUTE`: `{ATTRIBUTE_ID *id, XML_Bool isCdata, XML_Char *value}`
- Applied when element encountered if attribute not specified
- Invokes attribute handler with default value

---

### Content Model

**Scaffolding**:
- During `<!ELEMENT>` declaration, `CONTENT_SCAFFOLD` array built
- Represents tree structure: MIXED, CHOICE, SEQUENCE
- Functions:
  - **nextScaffoldPart(parser)** (line 600): Traverse scaffold
  - **build_model(parser)** (line 601): Convert scaffold to `XML_Content` tree

**Types**:
```c
enum XML_Content_Type {
  XML_CTYPE_EMPTY,      // <!ELEMENT foo EMPTY>
  XML_CTYPE_ANY,        // <!ELEMENT foo ANY>
  XML_CTYPE_MIXED,      // <!ELEMENT foo (#PCDATA | a | b)*>
  XML_CTYPE_NAME        // <!ELEMENT foo (a)>
}

enum XML_Content_Quant {
  XML_CQUANT_NONE,      // No quantifier: a
  XML_CQUANT_OPT,       // Optional: a?
  XML_CQUANT_REP,       // Zero or more: a*
  XML_CQUANT_PLUS       // One or more: a+
}
```

---

## 7. Error Handling

### Error Code Flow

**Enum XML_Error**:
- Defined in expat.h
- Includes: NO_MEMORY, INVALID_TOKEN, UNCLOSED_TOKEN, PARTIAL_CHAR, NO_ELEMENTS,
  ASYNC_ENTITY, SYNTAX_ERROR, etc.
- Special values for security: AMPLIFICATION_LIMIT_BREACH

**Error Flow**:
```
1. Processor function detects error condition
   → Sets parser->m_errorCode = XML_ERROR_*
   → Returns error code (not XML_ERROR_NONE)

2. callProcessor() receives error
   → Sets parser->m_processor = errorProcessor
   → Sets parser->m_eventEndPtr = parser->m_eventPtr

3. errorProcessor:
   → Subsequent calls return same error
   → No state change

4. User calls XML_GetErrorCode(parser)
   → Returns m_errorCode
   → Call XML_ErrorString(code) to get message
```

### Error Recovery

**No automatic recovery**:
- Once error state set, all parsing fails
- Must choose:
  - **Option 1**: Create new parser with `XML_ParserCreate()`
  - **Option 2**: Reset existing parser with `XML_ParserReset()`

**Error Position Tracking**:
- `parser->m_eventPtr`: Start of token/event causing error
- `parser->m_eventEndPtr`: End of event
- User can calculate line/column via `XML_GetCurrentLineNumber()`, `XML_GetCurrentColumnNumber()`

### Key Error Sources

**Tokenization**:
- `XML_TOK_INVALID` → `XML_ERROR_INVALID_TOKEN`
- `XML_TOK_PARTIAL` at end → `XML_ERROR_UNCLOSED_TOKEN`
- `XML_TOK_PARTIAL_CHAR` → `XML_ERROR_PARTIAL_CHAR`

**Semantic**:
- No root element → `XML_ERROR_NO_ELEMENTS`
- Entity closing at wrong depth → `XML_ERROR_ASYNC_ENTITY`
- Incomplete parameter entity → `XML_ERROR_INCOMPLETE_PE`

**Security**:
- Amplification exceeded → `XML_ERROR_AMPLIFICATION_LIMIT_BREACH`
- Memory limit → `XML_ERROR_NO_MEMORY`

---

## 8. Namespace Processing

### Namespace Mode

**Enabled via**:
- `XML_ParserCreateNS(encoding, nameSeparator)` — Create with NS support
- Default: nameSeparator = '|' (pipe character)
- Sets `parser->m_ns = XML_TRUE`

**In NS mode**:
- Element/attribute names: `namespace_uri|local_name`
- Namespace declarations tracked via `BINDING` chain
- Prefix resolution during parsing
- Namespace scope per-element

**In non-NS mode**:
- Names as-is, no processing
- Simpler `TAG_NAME` handling
- No binding/prefix structures

### Namespace Data Structures

**PREFIX** (lines 278-281):
```c
typedef struct prefix {
  const XML_Char *name;     // Prefix string
  BINDING *binding;         // Current active binding
} PREFIX;
```

**BINDING** (lines 268-276):
```c
typedef struct binding {
  struct prefix *prefix;                    // Back-pointer
  struct binding *nextTagBinding;           // Bindings in current tag
  struct binding *prevPrefixBinding;        // Previous binding for this prefix
  const struct attribute_id *attId;         // If declared via xmlns
  XML_Char *uri;                            // Namespace URI
  int uriLen;                               // URI length
  int uriAlloc;                             // Allocated space
} BINDING;
```

**TAG_NAME** (lines 283-290):
```c
typedef struct {
  const XML_Char *str;          // Full qualified name
  const XML_Char *localPart;    // Local part after separator
  const XML_Char *prefix;       // Prefix before separator
  int strLen;                   // Full name length
  int uriLen;                   // URI length
  int prefixLen;                // Prefix length
} TAG_NAME;
```

### Namespace Resolution

**Process**:
1. During element start tag parsing
2. Find all `xmlns` and `xmlns:prefix` attributes
3. Call **addBinding()** (line 524) for each
4. addBinding creates new BINDING:
   - Sets `binding->uri` to namespace URI
   - Links to prefix: `prefix->binding = binding`
   - Saves old binding: `binding->prevPrefixBinding = old`
5. Build TAG_NAME with prefix, localPart, full URI

**Scope**:
- Bindings linked to TAG structure
- When tag closes: call **freeBindings()** (line 519)
- Restores previous bindings: `prefix->binding = binding->prevPrefixBinding`

### Namespace Callbacks

**Declarations**:
```c
m_startNamespaceDeclHandler(handlerArg, prefix, uri)
m_endNamespaceDeclHandler(handlerArg, prefix)
```

**Default Namespace**:
- `xmlns="uri"` with no prefix
- Passed as `prefix = ""` or `prefix = NULL` to handlers

**Invocation Timing**:
- `startNamespaceDeclHandler`: Before element start handler
- `endNamespaceDeclHandler`: After element end handler

---

## 9. Key Function Categories

### Parser Lifecycle

- **XML_ParserCreate(encoding)** (1017): Create with defaults
- **XML_ParserCreateNS(encoding, separator)** (1022): Create with namespace support
- **XML_ParserCreate_MM(encoding, memSuite, nsSep)** (1338): Full creation with memory hooks
- **XML_ParserReset(parser, encoding)** (1642): Reset for reuse
- **XML_ParserFree(parser)** (1927): Destroy and free all resources
- **parserCreate()** (610, internal): Core creation logic
- **parserInit()** (615, internal): Initialization

### Main Parsing

- **XML_Parse(parser, data, len, isFinal)** (2343): Parse data directly
  - Copies data to buffer, calls XML_ParseBuffer
  - ~100 lines, handles buffer management
- **XML_ParseBuffer(parser, len, isFinal)** (2454): Parse buffered data
  - ~50 lines, main processor dispatch loop
  - Calls `callProcessor()` via `parser->m_processor`
- **startParsing(parser)** (608): Initialize parsing state

### Handler Setup

- **XML_SetStartElementHandler(parser, handler)**: Install start handler
- **XML_SetEndElementHandler(parser, handler)**: Install end handler
- **XML_SetCharacterDataHandler(parser, handler)**: Install text handler
- **XML_SetProcessingInstructionHandler(parser, handler)**: Install PI handler
- **XML_SetCommentHandler(parser, handler)**: Install comment handler
- **XML_SetStartCdataSectionHandler(parser, handler)**: Install CDATA start handler
- **XML_SetEndCdataSectionHandler(parser, handler)**: Install CDATA end handler
- **XML_SetDefaultHandler(parser, handler)**: Install default handler
- **XML_SetElementDeclHandler(parser, handler)**: Install element decl handler
- **XML_SetAttlistDeclHandler(parser, handler)**: Install attlist decl handler
- **XML_SetEntityDeclHandler(parser, handler)**: Install entity decl handler
- **XML_SetExternalEntityRefHandler(parser, handler)**: Install external entity handler
- **... and ~10 more for other callback types**
- All stored in `XML_ParserStruct`

### Error Handling

- **XML_GetErrorCode(parser)** (2748): Get error code
- **XML_ErrorString(code)** (2874): Convert code to string message
- **XML_GetCurrentLineNumber(parser)**: Get line of error
- **XML_GetCurrentColumnNumber(parser)**: Get column of error
- **XML_GetCurrentByteIndex(parser)**: Get byte position

### Parser Query

- **XML_GetUserData(parser)**: Get m_userData
- **XML_SetUserData(parser, data)**: Set m_userData
- **XML_GetBuffer(parser, len)**: Request buffer space
- **XML_ParsingStatus(parser)**: Get parsing state

### Utility Functions

- **normalizePublicId(s)** (564): Convert PUBLIC identifier to normal form
- **getContext(parser)** (561): Get namespace context
- **setContext(parser, context)** (562): Set namespace context
- **copyString(s, parser)** (605): Allocate and copy string

### Content Reporting

- **reportProcessingInstruction(parser, enc, start, end)** (554): Invoke PI handler
- **reportComment(parser, enc, start, end)** (556): Invoke comment handler
- **reportDefault(parser, enc, start, end)** (558): Invoke default handler

### DTD Utilities

- **getElementType(parser, enc, ptr, end)** (602): Find/create element type
- **defineAttribute(type, attId, isCdata, isId, value, parser)** (527): Add attribute
- **setElementTypePrefix(parser, elementType)** (541): Set namespace prefix
- **getAttributeId(parser, enc, start, end)** (539): Find/create attribute

### Tokenization & Attribute Handling

- **storeAtts(parser, enc, attStr, tagName, bindings, account)** (520): Parse attribute list
  - Calls `XmlParseAttlistDecl()` or similar
  - Invokes `storeAttributeValue()` for each
  - Returns error on syntax error
- **storeAttributeValue(parser, enc, isCdata, ptr, end, pool, account)** (530): Parse one attribute value
- **appendAttributeValue(parser, enc, isCdata, ptr, end, pool, account, nextPtr)** (536): Append to attribute
- **getAttributeId(parser, enc, start, end)** (539): Find/create attribute ID

### Entity Value Processing

- **storeEntityValue(parser, enc, start, end, account, nextPtr)** (543, XML_GE==1): Parse entity value
  - Processes references, resolves character/entity refs
- **callStoreEntityValue(parser, enc, start, end, account)** (547, XML_GE==1): Wrapper
- **storeSelfEntityValue(parser, entity)** (552, XML_GE==0): Self-referential entity value

### Security Accounting (XML_GE==1)

- **accountingGetCurrentAmplification(rootParser)** (618): Compute bytes_indirect / bytes_direct
- **accountingReportDiff(parser, diff, ...)** (621): Update byte counters
- **accountingReportStats(originParser, epilog)** (619): Print statistics
- **accountingOnAbort(originParser)** (620): Cleanup on error
- **accountingDiffTolerated(originParser, tok, before, after, line, account)** (626): Check limit
- **entityTrackingOnOpen/OnClose/ReportStats** (631-636): Track entity nesting

---

## 10. Proposed Rust Breakdown (8 Logical Modules)

### Design Principles
- **Layered architecture**: Lower layers have no dependencies on higher layers
- **Separation of concerns**: Each module has clear responsibility
- **Incremental porting**: Modules can be ported/tested independently
- **Idiomatic Rust**: Use Rust's type system, error handling, memory safety

---

### Module 1: Core Data Structures (ESSENTIAL)
**File**: `src/core_types.rs` (~500 lines)

**Purpose**: Define all data structures needed by other modules

**Contents**:
- `Parser` struct (Rust equivalent of `XML_ParserStruct`)
- `DTD`, `Entity`, `OpenInternalEntity`, `EntityType` structs
- `Binding`, `Prefix`, `Tag`, `TagName` structs
- `AttributeId`, `DefaultAttribute` structs
- `StringPool`, `HashTable` (structural definitions only, implementation in memory module)
- `PROLOG_STATE`, `Processor` type alias
- Error enum (`ParseError` mapping to `XML_Error`)
- Callback type definitions
- Constants and configuration

**Key Considerations**:
- Use Rust `Option<T>` for nullable pointers
- Use `Vec<T>` for arrays
- Use `Box<T>` for owned heap allocations
- Define traits for generic behavior
- Use lifetimes for borrowed references

---

### Module 2: String & Memory Management (FOUNDATIONAL)
**File**: `src/memory.rs` (~600 lines)

**Purpose**: Efficient memory allocation and string interning

**Contents**:
- `StringPool` implementation
  - `Block` structure for chunked allocation
  - `init()`, `clear()`, `destroy()`
  - `append()`, `store_string()`, `copy_string()`
  - `grow()` for expansion
  - Macro-like functions for pool manipulation

- `HashTable` implementation
  - Power-of-2 sizing
  - Double-hashing with `SECOND_HASH` and `PROBE_STEP` functions
  - `lookup()` for find-or-create
  - Iterator support
  - `init()`, `clear()`, `destroy()`

- Memory accounting helpers
  - Allocation tracking
  - Free list management for reusable structures

**Key Considerations**:
- Use Rust's `Allocator` trait for flexibility
- Implement `Iterator` trait for hash table
- Use `Arc<Mutex<>>` or `Rc<RefCell<>>` for shared pools
- Handle OOM gracefully

---

### Module 3: Tokenization & Encoding (FOUNDATIONAL)
**File**: `src/encoding.rs` (~300 lines)

**Purpose**: Wrapper around external tokenization (xmltok.h)

**Contents**:
- `Encoding` struct wrapper
  - Delegate to C tokenizer
  - Abstract encoding differences

- Token types (from xmltok.h)
  - `TokenType` enum
  - Token classification helpers

- Encoding initialization
  - BOM detection
  - Encoding declaration parsing
  - Encoding switching

- Character conversion
  - UTF-8 / UTF-16 / other
  - Internal encoding vs. document encoding

**Key Considerations**:
- FFI boundary to xmltok.c/xmlrole.c
- Use `extern "C"` for C function bindings
- Abstract encoding decisions
- Cache encoding info for performance

---

### Module 4: State Machine & Processors (CORE LOGIC)
**File**: `src/processor.rs` (~1200 lines)

**Purpose**: Parser state machine and processor functions

**Contents**:
- `Processor` trait (callable state handlers)
- Processor implementations (structs):
  - `PrologInitProcessor`
  - `PrologProcessor`
  - `ContentProcessor`
  - `CdataSectionProcessor`
  - `ExternalEntityInitProcessor` (chain)
  - `InternalEntityProcessor`
  - `EpilogProcessor`
  - `ErrorProcessor`
  - And others for DTD, entity values, etc.

- State transitions
  - Processor switching logic
  - State machine flow

- Common processor utilities
  - `call_processor()` dispatch
  - Error handling
  - Buffer management

**Key Considerations**:
- Use enum for processor type (avoid function pointers initially)
- Can refactor to trait objects later for performance
- Clear state transition documentation
- Test each processor independently

---

### Module 5: DTD & Element Declaration Processing (MAJOR)
**File**: `src/dtd.rs` (~1000 lines)

**Purpose**: DTD parsing and declaration processing

**Contents**:
- `do_prolog()` main function (~500 lines)
  - Tokenization with `XmlPrologTok()`
  - Validation with `XmlTokenRole()`
  - Declaration dispatch

- Declaration processors:
  - XML declaration: `process_xml_decl()`
  - DOCTYPE: `process_doctype_decl()`
  - Element: `process_element_decl()`
  - Attribute list: `process_attlist_decl()`
  - Entity: `process_entity_decl()`
  - Notation: `process_notation_decl()`
  - Parameter entity: `process_param_entity_ref()`

- DTD lifecycle:
  - `dtd_create()`
  - `dtd_reset()`
  - `dtd_destroy()`
  - `dtd_copy()`

- Content model:
  - `build_content_model()`
  - `next_scaffold_part()`
  - Convert scaffold to tree

- Element/attribute utilities:
  - `get_element_type()`
  - `define_attribute()`
  - `set_element_type_prefix()`
  - `get_attribute_id()`

**Key Considerations**:
- Heavy use of memory module (pools, hash tables)
- Callback invocation for handlers
- Complex state management
- Extensive error handling

---

### Module 6: Content & Entity Processing (MAJOR)
**File**: `src/content.rs` (~1200 lines)

**Purpose**: XML content parsing and entity expansion

**Contents**:
- `do_content()` main function (~400 lines)
  - Element tag processing
  - Text accumulation
  - Entity reference handling
  - CDATA section detection

- `do_cdata_section()`: CDATA processing
- `do_ignore_section()`: IGNORE section handling (XML_DTD)
- `process_entity()`: Entity expansion
- `process_xml_decl()`: XML declaration (duplicate in dtd.rs, shared logic)

- Attribute processing:
  - `store_atts()`: Parse attribute list
  - `store_attribute_value()`: Parse one attribute value
  - `append_attribute_value()`: Continuation

- Entity value processing (XML_GE==1):
  - `store_entity_value()`: Parse entity value
  - Character/entity reference resolution

- Namespace binding:
  - `add_binding()`: Create new namespace binding
  - `free_bindings()`: Release binding chain
  - Scope management

- Tag stack management:
  - Push/pop elements
  - Nesting depth tracking

**Key Considerations**:
- Heavy interaction with callback system
- Complex encoding handling
- Namespace binding chain management
- Error recovery in entity expansion

---

### Module 7: Callback & Event Reporting (INTEGRATION)
**File**: `src/callbacks.rs` (~400 lines)

**Purpose**: Callback handler storage and invocation

**Contents**:
- Callback handler struct
  - All 20+ handler function pointers
  - User data / handler arg separation

- Callback invocation wrappers:
  - `invoke_start_element()`
  - `invoke_end_element()`
  - `invoke_character_data()`
  - And similar for all handlers

- Event reporters:
  - `report_processing_instruction()`
  - `report_comment()`
  - `report_default()`

- Handler setters:
  - `set_start_element_handler()`
  - `set_character_data_handler()`
  - And similar for all handlers

- Default implementations:
  - No-op handlers
  - Handler chaining (if desired)

**Key Considerations**:
- Type-safe handler storage
- Encoding conversion before callback (source → API encoding)
- Error propagation from handlers
- Optional handlers (some may be null)

---

### Module 8: Parser Lifecycle & Main API (INTEGRATION)
**File**: `src/lib.rs` / `src/parser.rs` (~600 lines)

**Purpose**: Public API and parser lifecycle management

**Contents**:
- Public API functions (C FFI or pure Rust):
  - `XML_ParserCreate()`
  - `XML_ParserCreateNS()`
  - `XML_ParserCreate_MM()`
  - `XML_ParserReset()`
  - `XML_ParserFree()`

- Main parsing loop:
  - `XML_Parse()`
  - `XML_ParseBuffer()`

- Handler setters:
  - All `XML_SetXxxHandler()` functions

- Query functions:
  - `XML_GetErrorCode()`
  - `XML_GetErrorString()`
  - `XML_GetCurrentLineNumber()`
  - `XML_GetCurrentColumnNumber()`
  - `XML_GetUserData()`
  - `XML_SetUserData()`
  - `XML_GetBuffer()`
  - `XML_ParsingStatus()`

- Accounting queries (XML_GE==1):
  - Amplification tracking
  - Memory usage queries
  - Statistics

**Key Considerations**:
- Stable C FFI if intended for C compatibility
- Or clean Rust API if pure-Rust target
- Builder pattern for parser creation
- RAII for automatic cleanup
- Error type conversions

---

### Cross-Cutting Concerns

**Error Handling**:
- Define `ParseError` enum (all error codes)
- Use Rust `Result<T, ParseError>`
- Propagate via `?` operator
- Map to C error codes for FFI

**Security Accounting** (XML_GE==1):
- Amplification limit tracking
- Allocation limit enforcement
- Entity nesting depth monitoring
- Can be feature-gated (`#[cfg(feature = "accounting")]`)

**Memory Allocation**:
- Custom allocator support (needed for embedded systems)
- Tracking for accounting
- Free list management

**Namespace vs. Non-Namespace**:
- Feature flag or runtime flag
- Conditional compilation for NS-specific code
- Shared core, diverging attribute processing

**Encoding Handling**:
- UTF-8 default
- UTF-16 support (via xmltok.c)
- Unknown encoding callbacks
- BOM detection

---

## Summary

This comprehensive architecture analysis provides:

1. **Complete data structure catalog** with field descriptions and purposes
2. **State machine flow** with processor transitions and functions
3. **Memory management strategy** using pools and hash tables
4. **Callback system design** with all handler types and invocation patterns
5. **Entity expansion model** with open entity stack and recursion prevention
6. **DTD processing flow** with all declaration types
7. **Error handling approach** with error codes and recovery
8. **Namespace processing** with binding scopes and resolution
9. **Function categorization** by lifecycle, parsing, callbacks, utilities
10. **Proposed 8-module Rust breakdown** with clear responsibilities and dependencies

**Key insights for porting**:
- Core types must come first (Module 1)
- Memory management is foundational (Module 2)
- Processors are state machine handlers (Module 4)
- DTD and content parsing are the largest modules (Modules 5-6)
- Callbacks integrate handlers (Module 7)
- Lifecycle manages it all (Module 8)
- Encoding is an abstraction layer (Module 3)

**Porting priority** (for incremental approach):
1. Core types & error handling
2. Memory (string pools, hash tables)
3. Encoding wrapper
4. DTD (complex but critical)
5. Content (depends on DTD)
6. Processors (state machine)
7. Callbacks (event delivery)
8. Main API (ties everything together)
