use bevy::render::{
    mesh::{Indices, MeshVertexAttribute, MeshVertexAttributeId, VertexAttributeValues},
    render_resource::VertexFormat,
};

pub(crate) fn in_range(x: usize, bot: usize, top: usize) -> bool {
    return bot <= x && x < top;
}

pub(crate) trait VAVutils {
    fn extend(&mut self, t: &Self);
    fn filter_bool_array(&self, index_filter: Vec<bool>) -> Self;
    fn new(format: VertexFormat) -> VertexAttributeValues;
    fn offset_all(&self, offset: (usize, usize, usize)) -> VertexAttributeValues;
}

impl VAVutils for VertexAttributeValues {
    fn offset_all(&self, offset: (usize, usize, usize)) -> VertexAttributeValues {
        match self {
            VertexAttributeValues::Float32x3(vals) => {
                return VertexAttributeValues::Float32x3(
                    vals.iter()
                        .map(|[x, y, z]| {
                            [
                                x + offset.0 as f32,
                                y + offset.1 as f32,
                                z + offset.2 as f32,
                            ]
                        })
                        .collect(),
                );
            }
            _ => panic!("Method offset_all only works for Float32x3"),
        }
    }
    fn extend(&mut self, t: &VertexAttributeValues) {
        match (self, t) {
            (
                VertexAttributeValues::Float32(ref mut vals),
                VertexAttributeValues::Float32(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint32(ref mut vals),
                VertexAttributeValues::Sint32(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint32(ref mut vals),
                VertexAttributeValues::Uint32(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Float32x2(ref mut vals),
                VertexAttributeValues::Float32x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint32x2(ref mut vals),
                VertexAttributeValues::Sint32x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint32x2(ref mut vals),
                VertexAttributeValues::Uint32x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Float32x3(ref mut vals),
                VertexAttributeValues::Float32x3(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint32x3(ref mut vals),
                VertexAttributeValues::Sint32x3(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint32x3(ref mut vals),
                VertexAttributeValues::Uint32x3(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Float32x4(ref mut vals),
                VertexAttributeValues::Float32x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint32x4(ref mut vals),
                VertexAttributeValues::Sint32x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint32x4(ref mut vals),
                VertexAttributeValues::Uint32x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint16x2(ref mut vals),
                VertexAttributeValues::Sint16x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Snorm16x2(ref mut vals),
                VertexAttributeValues::Snorm16x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint16x2(ref mut vals),
                VertexAttributeValues::Uint16x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Unorm16x2(ref mut vals),
                VertexAttributeValues::Unorm16x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint16x4(ref mut vals),
                VertexAttributeValues::Sint16x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Snorm16x4(ref mut vals),
                VertexAttributeValues::Snorm16x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint16x4(ref mut vals),
                VertexAttributeValues::Uint16x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Unorm16x4(ref mut vals),
                VertexAttributeValues::Unorm16x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint8x2(ref mut vals),
                VertexAttributeValues::Sint8x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Snorm8x2(ref mut vals),
                VertexAttributeValues::Snorm8x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint8x2(ref mut vals),
                VertexAttributeValues::Uint8x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Unorm8x2(ref mut vals),
                VertexAttributeValues::Unorm8x2(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Sint8x4(ref mut vals),
                VertexAttributeValues::Sint8x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Snorm8x4(ref mut vals),
                VertexAttributeValues::Snorm8x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Uint8x4(ref mut vals),
                VertexAttributeValues::Uint8x4(values),
            ) => vals.extend(values),
            (
                VertexAttributeValues::Unorm8x4(ref mut vals),
                VertexAttributeValues::Unorm8x4(values),
            ) => vals.extend(values),
            _ => panic!("VertexAttributeValues must have the same variant"),
        }
    }

    fn new(format: VertexFormat) -> VertexAttributeValues {
        match format {
            VertexFormat::Float32 => VertexAttributeValues::Float32(vec![]),
            VertexFormat::Sint32 => VertexAttributeValues::Sint32(vec![]),
            VertexFormat::Uint32 => VertexAttributeValues::Uint32(vec![]),
            VertexFormat::Float32x2 => VertexAttributeValues::Float32x2(vec![]),
            VertexFormat::Sint32x2 => VertexAttributeValues::Sint32x2(vec![]),
            VertexFormat::Uint32x2 => VertexAttributeValues::Uint32x2(vec![]),
            VertexFormat::Float32x3 => VertexAttributeValues::Float32x3(vec![]),
            VertexFormat::Sint32x3 => VertexAttributeValues::Sint32x3(vec![]),
            VertexFormat::Uint32x3 => VertexAttributeValues::Uint32x3(vec![]),
            VertexFormat::Float32x4 => VertexAttributeValues::Float32x4(vec![]),
            VertexFormat::Sint32x4 => VertexAttributeValues::Sint32x4(vec![]),
            VertexFormat::Uint32x4 => VertexAttributeValues::Uint32x4(vec![]),
            VertexFormat::Sint16x2 => VertexAttributeValues::Sint16x2(vec![]),
            VertexFormat::Snorm16x2 => VertexAttributeValues::Snorm16x2(vec![]),
            VertexFormat::Uint16x2 => VertexAttributeValues::Uint16x2(vec![]),
            VertexFormat::Unorm16x2 => VertexAttributeValues::Unorm16x2(vec![]),
            VertexFormat::Sint16x4 => VertexAttributeValues::Sint16x4(vec![]),
            VertexFormat::Snorm16x4 => VertexAttributeValues::Snorm16x4(vec![]),
            VertexFormat::Uint16x4 => VertexAttributeValues::Uint16x4(vec![]),
            VertexFormat::Unorm16x4 => VertexAttributeValues::Unorm16x4(vec![]),
            VertexFormat::Sint8x2 => VertexAttributeValues::Sint8x2(vec![]),
            VertexFormat::Snorm8x2 => VertexAttributeValues::Snorm8x2(vec![]),
            VertexFormat::Uint8x2 => VertexAttributeValues::Uint8x2(vec![]),
            VertexFormat::Unorm8x2 => VertexAttributeValues::Unorm8x2(vec![]),
            VertexFormat::Sint8x4 => VertexAttributeValues::Sint8x4(vec![]),
            VertexFormat::Snorm8x4 => VertexAttributeValues::Snorm8x4(vec![]),
            VertexFormat::Uint8x4 => VertexAttributeValues::Uint8x4(vec![]),
            VertexFormat::Unorm8x4 => VertexAttributeValues::Unorm8x4(vec![]),
            _ => VertexAttributeValues::Float32x2(vec![]),
        }
    }
    fn filter_bool_array(&self, index_filter: Vec<bool>) -> VertexAttributeValues {
        match self {
            VertexAttributeValues::Float32(values) => Self::Float32(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<f32>>(),
            ),
            VertexAttributeValues::Sint32(values) => Self::Sint32(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<i32>>(),
            ),
            VertexAttributeValues::Uint32(values) => Self::Uint32(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<u32>>(),
            ),
            VertexAttributeValues::Float32x2(values) => Self::Float32x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[f32; 2]>>(),
            ),
            VertexAttributeValues::Sint32x2(values) => Self::Sint32x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i32; 2]>>(),
            ),
            VertexAttributeValues::Uint32x2(values) => Self::Uint32x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u32; 2]>>(),
            ),
            VertexAttributeValues::Float32x3(values) => Self::Float32x3(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[f32; 3]>>(),
            ),
            VertexAttributeValues::Sint32x3(values) => Self::Sint32x3(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i32; 3]>>(),
            ),
            VertexAttributeValues::Uint32x3(values) => Self::Uint32x3(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u32; 3]>>(),
            ),
            VertexAttributeValues::Float32x4(values) => Self::Float32x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[f32; 4]>>(),
            ),
            VertexAttributeValues::Sint32x4(values) => Self::Sint32x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i32; 4]>>(),
            ),
            VertexAttributeValues::Uint32x4(values) => Self::Uint32x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u32; 4]>>(),
            ),
            VertexAttributeValues::Sint16x2(values) => Self::Sint16x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i16; 2]>>(),
            ),
            VertexAttributeValues::Snorm16x2(values) => Self::Snorm16x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i16; 2]>>(),
            ),
            VertexAttributeValues::Uint16x2(values) => Self::Uint16x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u16; 2]>>(),
            ),
            VertexAttributeValues::Unorm16x2(values) => Self::Unorm16x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u16; 2]>>(),
            ),
            VertexAttributeValues::Sint16x4(values) => Self::Sint16x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i16; 4]>>(),
            ),
            VertexAttributeValues::Snorm16x4(values) => Self::Snorm16x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i16; 4]>>(),
            ),
            VertexAttributeValues::Uint16x4(values) => Self::Uint16x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u16; 4]>>(),
            ),
            VertexAttributeValues::Unorm16x4(values) => Self::Unorm16x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u16; 4]>>(),
            ),
            VertexAttributeValues::Sint8x2(values) => Self::Sint8x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i8; 2]>>(),
            ),
            VertexAttributeValues::Snorm8x2(values) => Self::Snorm8x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i8; 2]>>(),
            ),
            VertexAttributeValues::Uint8x2(values) => Self::Uint8x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u8; 2]>>(),
            ),
            VertexAttributeValues::Unorm8x2(values) => Self::Unorm8x2(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u8; 2]>>(),
            ),
            VertexAttributeValues::Sint8x4(values) => Self::Sint8x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i8; 4]>>(),
            ),
            VertexAttributeValues::Snorm8x4(values) => Self::Snorm8x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[i8; 4]>>(),
            ),
            VertexAttributeValues::Uint8x4(values) => Self::Uint8x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u8; 4]>>(),
            ),
            VertexAttributeValues::Unorm8x4(values) => Self::Unorm8x4(
                values
                    .iter()
                    .enumerate()
                    .filter(|&(i, x)| index_filter[i])
                    .map(|(_, &x)| x)
                    .collect::<Vec<[u8; 4]>>(),
            ),
        }
    }
}
