This project is currently focused on exposing the top level USD api and the schemas on top of
those as much as possible. Everything related to UsdStage, UsdPrim and where necessary the
lower level types should be exposed, such as VtValue.

The intention is that later on the higher performance sdf and hydra apis are also exposed.

# Progress
This is a checklist of the state of exposed objects in the
USD libraries. If the library is not listed then nothing is
exposed.

## sdf

|            Class               |     Usable       |
|--------------------------------|------------------|

## tf

|            Class               |     Usable       |
|--------------------------------|------------------|

## usd

|            Class               |     Usable       |
|--------------------------------|------------------|
|  EndSentinel                   |                  |
|  Iterator                      |                  |
|  LayerMutingChanged            |                  |
|  ObjectsChanged                |                  |
|  PcpPrimIndex                  |                  |
|  StageContentsChanged          |                  |
|  StageEditTargetChanged        |                  |
|  StageNotice                   |                  |
|  UsdAPISchemaBase              |                  |
|  UsdAttribute                  |:heavy_check_mark:|
|  UsdClipsAPI                   |                  |
|  UsdCollectionAPI              |                  |
|  UsdEditTarget                 |                  |
|  UsdExpiredPrimAccessError     |                  |
|  UsdInherits                   |                  |
|  UsdModelAPI                   |                  |
|  UsdNotice                     |                  |
|  UsdObject                     |                  |
|  UsdPayloads                   |                  |
|  UsdPrim                       |:heavy_check_mark:|
|  Usd_PrimData                  |                  |
|  Usd_PrimDataSiblingIterator   |                  |
|  Usd_PrimDataSubtreeIterator   |                  |
|  Usd_PrimFlagsConjunction      |                  |
|  Usd_PrimFlagsDisjunction      |                  |
|  UsdPrimSiblingIterator        |                  |
|  UsdPrimSiblingRange           |                  |
|  UsdPrimSubtreeIterator        |                  |
|  UsdPrimSubtreeRange           |                  |
|  UsdPrimTypeInfo               |                  |
|  UsdProperty                   |                  |
|  UsdReferences                 |:heavy_check_mark:|
|  UsdRelationship               |:heavy_check_mark:|
|  Usd_Resolver                  |                  |
|  UsdResolveTarget              |                  |
|  UsdSchemaBase                 |                  |
|  UsdSchemaKind                 |                  |
|  UsdSchemaRegistry             |                  |
|  UsdSpecializes                |                  |
|  UsdStage                      |:heavy_check_mark:|
|  UsdTimeCode                   |                  |
|  UsdTyped                      |                  |
|  UsdUsdaFileFormat             |                  |
|  UsdUsdcFileFormat             |                  |
|  UsdUsdFileFormat              |                  |
|  UsdUsdzFileFormat             |                  |
|  UsdVariantSet                 |                  |
|  UsdVariantSets                |                  |
|  VersionPolicy                 |                  |

## usd_geom

|            Class               |     Usable       |
|--------------------------------|------------------|

## usd_shade

|            Class               |     Usable       |
|--------------------------------|------------------|

## tf

|            Class               |     Usable       |
|--------------------------------|------------------|


