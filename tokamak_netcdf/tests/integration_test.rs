use tokamak_netcdf::NcData;

mod common;

#[test]
fn test_nc_data_creation() -> Result<(), netcdf::Error> {
    let path = &common::phony_netcdf_path()?;
    let nc_data = NcData::open(path.into());

    assert!(nc_data.is_ok());

    // test for functionality
    let _ = format!("{:?}", nc_data);
    let _ = format!("{:#?}", nc_data);

    std::fs::remove_file(path).expect("Failed to delete temp phony.nc");
    Ok(())
}
