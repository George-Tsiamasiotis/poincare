use tokamak_netcdf::NcData;

mod common;

#[test]
fn test_nc_data_creation() -> Result<(), netcdf::Error> {
    let path = &common::phony_netcdf_path()?;
    let nc_data = NcData::open(path.into()).unwrap();

    // test for functionality
    let _ = format!("{:?}", nc_data);
    let _ = format!("{:#?}", nc_data);

    std::fs::remove_file(path).unwrap();
    Ok(())
}
