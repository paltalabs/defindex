import { fetchFactoryAddress } from "./factory";

describe('fetchFactoryAddress', () => {
    it('should fetch the factory address on testnet', async () => {
        // Mock the fetch function
        const mockResponse = {
            json: jest.fn().mockResolvedValue({
                "ids": {
                    "defindex_factory": "CCBQ4WFNNWZWV7GCLUTRAQZXX34WSYNSCXMY4WEKV7BXRTYMM53JMGIS"
                },
                "hashes": {
                    "defindex_vault": "6ac83e448a7a68b4e27fc9dd3c2d3e8cb423d88e2646137b7b32238e9df95980",
                    "defindex_factory": "552b67db3c57921fada99cfee520f18a649855a376c22e8cb5ee20b52632d497"
                }
            }),
        };
        global.fetch = jest.fn().mockResolvedValue(mockResponse);

        // Call the function
        const factoryAddress = await fetchFactoryAddress("testnet");

        // Assertions
        expect(fetch).toHaveBeenCalledWith('https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/testnet.contracts.json');
        expect(mockResponse.json).toHaveBeenCalled();
        expect(factoryAddress).toBe('CCBQ4WFNNWZWV7GCLUTRAQZXX34WSYNSCXMY4WEKV7BXRTYMM53JMGIS');
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