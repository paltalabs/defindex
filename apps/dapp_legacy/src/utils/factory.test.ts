import { fetchFactoryAddress } from "./factory";

describe('fetchFactoryAddress', () => {
    it('should fetch the factory address on testnet', async () => {
        // Mock the fetch function
        const mockResponse = {
            json: jest.fn().mockResolvedValue({
                blend_strategy: "CCFT4EVTUSYUNO7CYJHC3R2F5ZBTXW7CMJW5Z2PGVYCHWNDG4RS35YZ5",
                defindex_factory:"CAJJAIJT6E7GMJKFA66RRI7KUNNR22TPNLNUFCNANGCFXJ54RYPVPPJT",
                fixed_apr_strategy:"CDR6K2L2UN3SZLBOUCJCVNKKKGC4F5DHGQ6QIPVBXU3UTXTTGWWEZ2H3",
                hodl_strategy:"CDTSVTAI4BXYIEZ66F2TLZ337OLW5R5P4ONWMHQY5XTOTBZURZEDZ64N"
            }),
        };
        global.fetch = jest.fn().mockResolvedValue({
            json: jest.fn().mockResolvedValue(mockResponse.json())
        });

        // Call the function
        const factoryAddress = await fetchFactoryAddress("testnet");

        // Assertions
        expect(mockResponse.json).toHaveBeenCalled();
        expect(factoryAddress).toHaveLength(56);
    });

    it('should throw an error if the response status is 404', async () => {
        // Mock the fetch function to return a 404 status
        const mockResponse = {
            status: 404,
            json: jest.fn().mockResolvedValue({}),
        };
        global.fetch = jest.fn().mockResolvedValue(mockResponse);

        // Call the function and assert that it throws an error
        try {
            await fetchFactoryAddress("mainnet");
        } catch (error) {
            expect(error).toEqual(new Error('Deployment not found for network: mainnet'));
        }
    });
});