package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase

/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object url
     
    /**
     * <p></p>
     */
    public static Object username
     
    /**
     * <p></p>
     */
    public static Object password
     

    static {
        def allVariables = [:]        
        allVariables.put('default', ['url' : 'https://opensource-demo.orangehrmlive.com/', 'username' : 'Admin', 'password' : 'admin123'])
        allVariables.put('QA', allVariables['default'] + ['url' : 'https://opensource-demo.orangehrmlive.com/', 'username' : 'Adminqa', 'password' : 'admin123'])
        allVariables.put('Regres', allVariables['default'] + ['url' : 'https://opensource-demo.orangehrmlive.com/', 'username' : 'Adminregr', 'password' : 'admin123'])
        allVariables.put('Stg', allVariables['default'] + ['url' : 'https://opensource-demo.orangehrmlive.com/', 'username' : 'Adminstg', 'password' : 'admin123'])
        
        String profileName = RunConfiguration.getExecutionProfile()
        
        def selectedVariables = allVariables[profileName]
        url = selectedVariables['url']
        username = selectedVariables['username']
        password = selectedVariables['password']
        
    }
}
